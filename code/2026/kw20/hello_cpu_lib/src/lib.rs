// src/lib.rs
use raw_cpuid::CpuId;
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::ptr::copy_nonoverlapping;
//
// LabVIEW LStr definition (from extcode.h)

#[repr(C)]
pub struct LStr {
    pub cnt: i32,     // number of bytes
    pub str: [u8; 1], // flexible array member
}

pub type LStrPtr = *mut LStr;
pub type LStrHandle = *mut LStrPtr;

//
// LabVIEW Memory Manager
//
#[link(name = "labview")]
unsafe extern "C" {
    fn DSSetHandleSize(handle: *mut c_void, new_size: usize) -> i32;
}

//
// Helper: get CPU brand string
//
use encoding_rs::WINDOWS_1252;

fn get_cpu_brand_bytes() -> Vec<u8> {
    let cpuid = CpuId::new();
    let s = cpuid
        .get_processor_brand_string()
        .map(|b| b.as_str().trim_end_matches('\0').to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());

    // Replace ASCII markers with Unicode symbols
    let unicode = s
        .replace("(R)", "®")
        .replace("(TM)", "™");

    // Encode to Windows‑1252 (ANSI)
    let (encoded, _, _) = WINDOWS_1252.encode(&unicode);

    encoded.into_owned() // <-- raw ANSI bytes
}
//
// 1) Fill preallocated C string buffer
//
#[unsafe(no_mangle)]
pub extern "C" fn cpu_string_c(buffer: *mut c_char, length: c_int) {
    if buffer.is_null() || length <= 0 {
        return;
    }

    let bytes = get_cpu_brand_bytes();

    let max = (length - 1) as usize; // leave room for null terminator
    let n = bytes.len().min(max);

    unsafe {
        copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, n);
        *buffer.add(n) = 0; // null terminate
    }
}

//
// 2) Fill LabVIEW LStrHandle (auto-resize)
//
#[unsafe(no_mangle)]
pub extern "C" fn cpu_string_lv(h: LStrHandle) -> i32 {
    if h.is_null() {
        return 1; // mgArgErr
    }

    let bytes = get_cpu_brand_bytes();
    let size = bytes.len();

    unsafe {
        // Resize handle: size = length + sizeof(int32)
        let err = DSSetHandleSize(h as *mut c_void, size + 4);
        if err != 0 {
            return err;
        }

        let ptr = *h;
        if ptr.is_null() {
            return 1;
        }

        (*ptr).cnt = size as i32;

        let dst = (*ptr).str.as_mut_ptr();
        copy_nonoverlapping(bytes.as_ptr(), dst, size);
    }
    0 // mgNoErr
}
