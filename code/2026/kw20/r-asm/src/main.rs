use core::arch::x86_64::*;
use std::arch::asm;

fn main() {
    unsafe {
        // Prepare input vectors
        let a = _mm256_set1_epi32(10);
        let b = _mm256_set1_epi32(20);

        // Output vector
        let mut result = _mm256_setzero_si256();

        // Inline AVX2 assembly
        asm!(
            "vmovdqu ymm1, [{a_ptr}]",
            "vmovdqu ymm2, [{b_ptr}]",
            "vpaddd ymm3, ymm1, ymm2",
            "vmovdqu [{out_ptr}], ymm3",

            a_ptr = in(reg) &a,
            b_ptr = in(reg) &b,
            out_ptr = in(reg) &mut result,

            options(nostack, preserves_flags)
        );

        // Dump registers into arrays for printing
        let mut out = [0i32; 8];
        let mut a_t = [0i32; 8];
        let mut b_t = [0i32; 8];

        _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
        _mm256_storeu_si256(a_t.as_mut_ptr() as *mut __m256i, a);
        _mm256_storeu_si256(b_t.as_mut_ptr() as *mut __m256i, b);

        println!("a_t  = {:?}", a_t);
        println!("b_t  = {:?}", b_t);
        println!("out  = {:?}", out);
    }
}
