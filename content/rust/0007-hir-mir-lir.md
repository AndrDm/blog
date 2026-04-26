---
title: HIR MIR LIR (LLVM IR)
date: 2026-04-26
description: HIR
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

Time to time, and especially for learning purposes, it’s useful to inspect the internal representations that the Rust compiler generates. Below are some notes on how to do that.

<!--more-->

First, update Rust to ensure you have the latest nightly toolchain:

```
rustup update
```

Install log:

```
info: syncing channel updates for stable-x86_64-pc-windows-msvc
info: syncing channel updates for nightly-x86_64-pc-windows-msvc
info: latest update on 2026-04-26 for version 1.97.0-nightly (9838411cb 2026-04-25)
info: downloading 7 components
     rust-src installed                        3.85 MiB
        cargo installed                        9.70 MiB
       clippy installed                        3.92 MiB
    rust-docs installed                       22.49 MiB
     rust-std installed                       21.44 MiB
        rustc installed                       68.29 MiB
      rustfmt installed                        2.47 MiB
  stable-x86_64-pc-windows-msvc unchanged - rustc 1.95.0 (59807616e 2026-04-14)
   nightly-x86_64-pc-windows-msvc updated - rustc 1.97.0-nightly (9838411cb 2026-04-25)

info: checking for self-update (current version: 1.29.0)
info: cleaning up downloads & tmp directories
```

Now we need a very simple Rust code

```Rust
fn main() {
    /*
    Immutable Borrow:
    - print_value receives &i32, a reference to x.
    - Ownership stays with main.
    - x is still usable afterward because it was only borrowed, not moved.
    */

    let x = 10;
    print_value(&x); // borrow x
    println!("x is still usable: {}", x);
       
    /*
    Mutable Borrow (allows modification):
    - &mut i32 gives temporary permission to modify y.
    - Only one mutable borrow is allowed at a time.
    - After the borrow ends, y is usable again.
    Remember
    Rust's primitive integers (i32, u32, etc.) implement the Copy trait.
    This means:
    let a = 5;
    let b = a; // copies, does NOT move
    */

    let mut y = 10;
    add_one(&mut y); // mutable borrow    
    println!("y after modification: {}", y);
}

fn print_value(n: &i32) {
    println!("The value is {}", n);
}

fn add_one(n: &mut i32) {
    *n += 1; // dereference and modify
    println!("The new value is {}", n);
}
```

To inspect the compiler’s High‑Level Intermediate Representation (HIR), run:

```
cargo +nightly rustc -- -Zunpretty=hir > hir.log
```

This writes the HIR output into `hir.log`.

## Example HIR Output

HIR is desugared, so we’ll see:

- explicit calls to `std::io::_print`
- lowered formatting macros
- explicit lifetime annotations (`'_'`)
- desugared blocks and scopes

This is expected — HIR is not meant to look like your original Rust code; it’s the compiler’s cleaned‑up, type‑checked representation:

```rust
extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2024::*;
fn main() {
    /*
    Immutable Borrow:
    - print_value receives &i32, a reference to x.
    - Ownership stays with main.
    - x is still usable afterward because it was only borrowed, not moved.
    */

    let x = 10;
    print_value(&x); // borrow x

    /*
    Mutable Borrow (allows modification):
    - &mut i32 gives temporary permission to modify y.
    - Only one mutable borrow is allowed at a time.
    - After the borrow ends, y is usable again.
    Remember
    Rust's primitive integers (i32, u32, etc.) implement the Copy trait.
    This means:
    let a = 5;
    let b = a; // copies, does NOT move
    */

    // mutable borrow    


    // dereference and modify
    {
        ::std::io::_print({
                super let args = (&x,);
                super let args = [format_argument::new_display(args.0)];
                unsafe {
                    format_arguments::new(b"\x13x is still usable: \xc0\x01\n\x00",
                        &args)
                }
            });
    };
    let mut y = 10;
    add_one(&mut y);
    {
        ::std::io::_print({
                super let args = (&y,);
                super let args = [format_argument::new_display(args.0)];
                unsafe {
                    format_arguments::new(b"\x16y after modification: \xc0\x01\n\x00",
                        &args)
                }
            });
    };
}
fn print_value(n:
        &'_ i32) {
    {
        ::std::io::_print({
                super let args = (&n,);
                super let args = [format_argument::new_display(args.0)];
                unsafe {
                    format_arguments::new(b"\rThe value is \xc0\x01\n\x00",
                        &args)
                }
            });
    };
}
fn add_one(n:
        &'_ mut i32) {
    *n += 1;
    {
        ::std::io::_print({
                super let args = (&n,);
                super let args = [format_argument::new_display(args.0)];
                unsafe {
                    format_arguments::new(b"\x11The new value is \xc0\x01\n\x00",
                        &args)
                }
            });
    };
}
```

