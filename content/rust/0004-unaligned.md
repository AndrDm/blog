---
title: Unaligned Memory in Rust - Forcing Odd Addresses and Loading Data Safely
date: 2026-04-10
authorbox: false
sidebar: false
description: How to Safely Work With Unaligned Memory in Rust - Odd Addresses, Raw Pointers, and No UB
programming_languages:
  - Rust
categories:
  - Programming
archives:
  - 2026-04
tags:
  - MyLearning
draft: false

---
Rust provides strong guarantees about memory safety, alignment, and the behavior of references. However, when you step into the world of low‑level programming — manual allocation, pointer arithmetic, and raw memory manipulation — it becomes your job to uphold these guarantees.
One interesting challenge:
Can we intentionally place a u32 at an odd address and still read it safely?
<!--more-->
In this article, we explore exactly that. We allocate raw memory, force an odd address, store a struct at that location, and then safely load an unaligned 32‑bit value.

<iframe src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=18b2b653ae6052a5d0f723e0efc5edd0" width="1200" height="1000" allowfullscreen frameBorder="0" scrolling="no"></iframe>


<details>
  <summary>Code Snippet</summary>
```Rust
use std::alloc::{alloc, Layout};
use std::ptr::addr_of;

#[repr(C, align(1))]
struct Aligned1 {
    b: u32,
}

fn main() {

    // 1) Allocate raw memory — at least sizeof(struct)+1 bytes
    //
    // We add +1 byte so we can later shift the pointer by 1,
    // ensuring the resulting address is guaranteed to be odd.
    let layout = Layout::from_size_align(std::mem::size_of::<Aligned1>() + 1, 1).unwrap();
    let raw = unsafe { alloc(layout) };

    // 2) Make the address ODD by shifting it by 1 byte
    //
    // Raw allocation may return an even or odd address (usually even).
    // Adding 1 forces the resulting pointer to have the lowest bit = 1.
    let odd_ptr = unsafe { raw.add(1) };

    // 3) Store the struct at the odd address
    //
    // This is an *unaligned write* — allowed because we are writing raw bytes,
    // not creating a reference to unaligned data.
    unsafe { odd_ptr.cast::<Aligned1>().write(Aligned1 { b: 0x11223344 }); }

    let p: *const Aligned1 = odd_ptr.cast();

    // 4) Obtain the address of field `b`
    //
    // addr_of! does NOT create a reference (&T), so it is safe even when unaligned.
    let ptr_b = unsafe { addr_of!((*p).b) };

    println!("addr(struct) = {:p}", p);
    println!("addr(b)      = {:p}", ptr_b);
    println!("addr(b) % 2  = {}", (ptr_b as usize) % 2); // should be 1

    unsafe {
        // Direct dereference of *ptr_b WOULD CRASH:
        //
        //   panic: misaligned pointer dereference
        //
        // because loading a `u32` from a non‑4‑byte-aligned address is illegal
        // on most CPUs.
        // Uncomment to get crash:
        // println!("b via raw  = 0x{:X}", *ptr_b);
    }

    // -------------------------------------------------------------------------
    // Crash-free version
    // -------------------------------------------------------------------------

    // 1) Allocate raw memory again (same method)
    let layout = Layout::from_size_align(std::mem::size_of::<Aligned1>() + 1, 1).unwrap();
    let raw = unsafe { alloc(layout) };

    // 2) Shift by 1 byte → guaranteed ODD address
    let odd_ptr = unsafe { raw.add(1) };

    // 3) Unaligned store
    unsafe { odd_ptr.cast::<Aligned1>().write(Aligned1 { b: 0x11223344 }); }

    // 4) Pointer to the struct and pointer to the field
    let p = odd_ptr.cast::<Aligned1>();
    let ptr_b = unsafe { addr_of!((*p).b) };

    println!("addr(b) = {:p}", ptr_b);

    // ✅ 5) Read the 4 bytes manually into a byte array.
    //
    // This avoids any alignment requirements because reading `u8` is always legal.
    let bytes: [u8; 4] = unsafe {
        [
            *ptr_b.cast::<u8>().add(0),
            *ptr_b.cast::<u8>().add(1),
            *ptr_b.cast::<u8>().add(2),
            *ptr_b.cast::<u8>().add(3),
        ]
    };

    // ✅ 6) Reassemble the u32 from bytes without invoking undefined behavior.
    //
    // This is the correct and portable way to load unaligned integers.
    let value = u32::from_ne_bytes(bytes);
    println!("value = 0x{:X}", value);
}
```

</details>

