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

## A side‑by‑side comparison: **Rust → HIR → THIR → MIR → LLVM IR**

Minimal Example:

```rust
fn add_one(n: i32) -> i32 {
    n + 1
}

fn main() {
    let x = 10;
    let y = add_one(x);
    println!("y = {}", y);
}
```

### HIR — High‑Level Intermediate Representation

```
cargo +nightly rustc -- -Zunpretty=hir > hir.log
```

Output

```rust
extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2024::*;
fn add_one(n: i32) -> i32 { n + 1 }

fn main() {
    let x = 10;
    let y = add_one(x);
    {
        ::std::io::_print({
                super let args = (&y,);
                super let args = [format_argument::new_display(args.0)];
                unsafe {
                    format_arguments::new(b"\x04y = \xc0\x01\n\x00", &args)
                }
            });
    };
}
```

### THIR — Typed High‑Level Intermediate Representation

```
cargo +nightly rustc -- -Zunpretty=thir-tree > thir.log
```

Output

```
DefId(0:3 ~ r_hir2[f5f1]::add_one):
params: [
    Param {
        ty: i32
        ty_span: Some(src\main.rs:1:15: 1:18 (#0))
        self_kind: None
        hir_id: Some(HirId(DefId(0:3 ~ r_hir2[f5f1]::add_one).1))
        param: Some( 
            Pat {
                ty: i32
                span: src\main.rs:1:12: 1:13 (#0)
                kind: PatKind {
                    Binding {
                        name: "n"
                        mode: BindingMode(No, Not)
                        var: LocalVarId(HirId(DefId(0:3 ~ r_hir2[f5f1]::add_one).2))
                        ty: i32
                        is_primary: true
                        is_shorthand: false
                        subpattern: None
                    }
                }
            }
        )
    }
]
body:
    Expr {
        ty: i32
        temp_scope_id: 8
// ... + ~700 lines
```

### MIR  — Mid‑Level Intermediate Representation

(with borrow regions, moves, drops, etc.)

```
cargo +nightly rustc -- -Zunpretty=mir > mir.log
```

Output, this one is useful

```rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
// HINT: See also -Z dump-mir for MIR at specific points during compilation.
fn add_one(_1: i32) -> i32 {
    debug n => _1;
    let mut _0: i32;
    let mut _2: (i32, bool);

    bb0: {
        _2 = AddWithOverflow(copy _1, const 1_i32);
        assert(!move (_2.1: bool), "attempt to compute `{} + {}`, which would overflow", copy _1, const 1_i32) -> [success: bb1, unwind continue];
    }

    bb1: {
        _0 = move (_2.0: i32);
        return;
    }
}

fn main() -> () {
    let mut _0: ();
    let mut _2: i32;
    let _3: ();
    let mut _4: std::fmt::Arguments<'_>;
    let mut _6: &i32;
    let mut _8: core::fmt::rt::Argument<'_>;
    let mut _9: &[u8; 9];
    let _10: &[core::fmt::rt::Argument<'_>; 1];
    let mut _11: &i32;
    scope 1 {
        debug x => const 10_i32;
        let _1: i32;
        scope 2 {
            debug y => _1;
            let _5: (&i32,);
            scope 3 {
                debug args => _5;
                let _7: [core::fmt::rt::Argument<'_>; 1];
                scope 4 {
                    debug args => _7;
                }
            }
        }
    }

    bb0: {
        _2 = const 10_i32;
        _1 = add_one(move _2) -> [return: bb1, unwind continue];
    }

    bb1: {
        _6 = &_1;
        _5 = (move _6,);
        _11 = copy (_5.0: &i32);
        _8 = core::fmt::rt::Argument::<'_>::new_display::<i32>(copy _11) -> [return: bb2, unwind continue];
    }

    bb2: {
        _7 = [move _8];
        _9 = const b"\x04y = \xc0\x01\n\x00";
        _10 = &_7;
        _4 = Arguments::<'_>::new::<9, 1>(move _9, copy _10) -> [return: bb3, unwind continue];
    }

    bb3: {
        _3 = std::io::_print(move _4) -> [return: bb4, unwind continue];
    }

    bb4: {
        return;
    }
}

alloc1 (size: 9, align: 1) {
    04 79 20 3d 20 c0 01 0a 00                      │ .y = ....
}
```

### LLVM IR — Low‑Level Virtual Machine Intermediate Representation

```
cargo +nightly rustc --release -- --emit=llvm-ir
```

Low level

```
; ModuleID = 'r_hir2.43a4cb817f29a3bb-cgu.0'
source_filename = "r_hir2.43a4cb817f29a3bb-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0INtNtNtCslGnHuDJZtoO_4core3ops8function6FnOnceuE9call_once6vtableCs5O45EZMTWYx_6r_hir2, ptr @_RNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0Cs5O45EZMTWYx_6r_hir2, ptr @_RNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0Cs5O45EZMTWYx_6r_hir2 }>, align 8
@alloc_38f7561a1e08933acd4594936de9b8a4 = private unnamed_addr constant [9 x i8] c"\04y = \C0\01\0A\00", align 1

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuECs5O45EZMTWYx_6r_hir2(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCs3xbTYQS5Gwi_3std2rt19lang_start_internal(ptr noundef nonnull %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(ptr nonnull %_7)
  ret i64 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs3xbTYQS5Gwi_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5O45EZMTWYx_6r_hir2(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #1 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #6, !srcloc !3
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0Cs5O45EZMTWYx_6r_hir2(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs3xbTYQS5Gwi_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5O45EZMTWYx_6r_hir2(ptr noundef nonnull %_4) #7
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0INtNtNtCslGnHuDJZtoO_4core3ops8function6FnOnceuE9call_once6vtableCs5O45EZMTWYx_6r_hir2(ptr noundef readonly captures(none) %_1) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs3xbTYQS5Gwi_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5O45EZMTWYx_6r_hir2(ptr noundef nonnull readonly %0) #7, !noalias !5
  ret i32 0
}

; r_hir2::main
; Function Attrs: uwtable
define hidden void @_RNvCs5O45EZMTWYx_6r_hir24main() unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  %y = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(ptr nonnull %y)
  store i32 11, ptr %y, align 4
  call void @llvm.lifetime.start.p0(ptr nonnull %args)
  store ptr %y, ptr %args, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs9_NtNtNtCslGnHuDJZtoO_4core3fmt3num3implNtB9_7Display3fmt, ptr %_6.sroa.4.0..sroa_idx, align 8
; call std::io::stdio::_print
  call void @_RNvNtNtCs3xbTYQS5Gwi_3std2io5stdio6__print(ptr noundef nonnull @alloc_38f7561a1e08933acd4594936de9b8a4, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(ptr nonnull %args)
  call void @llvm.lifetime.end.p0(ptr nonnull %y)
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(ptr captures(none)) #3

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCs3xbTYQS5Gwi_3std2rt19lang_start_internal(ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(ptr captures(none)) #3

; <i32 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs9_NtNtNtCslGnHuDJZtoO_4core3fmt3num3implNtB9_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs3xbTYQS5Gwi_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

declare i32 @__CxxFrameHandler3(...) unnamed_addr #4

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #5 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(ptr nonnull %_7.i)
  store ptr @_RNvCs5O45EZMTWYx_6r_hir24main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs3xbTYQS5Gwi_3std2rt19lang_start_internal(ptr noundef nonnull %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

attributes #0 = { uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #1 = { noinline uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #2 = { inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { "target-cpu"="x86-64" }
attributes #5 = { "target-cpu"="x86-64" "target-features"="+cx16,+sse,+sse2,+sse3,+sahf" }
attributes #6 = { nounwind }
attributes #7 = { noinline }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.97.0-nightly (9838411cb 2026-04-25)"}
!3 = !{i64 5317075024405431}
!4 = !{}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0Cs5O45EZMTWYx_6r_hir2: %_1"}
!7 = distinct !{!7, !"_RNCINvNtCs3xbTYQS5Gwi_3std2rt10lang_startuE0Cs5O45EZMTWYx_6r_hir2"}

```

### ASM

```
cargo rustc --release -- --emit=asm -C "llvm-args=-x86-asm-syntax=intel"
```

Output:

```
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
@feat.00 = 0
	.intel_syntax noprefix
	.file	"r_hir2.e4ca531aada78c05-cgu.0"
	.def	_ZN3std2rt10lang_start17h664582d5029fc6c8E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17h664582d5029fc6c8E,unique,0
	.globl	_ZN3std2rt10lang_start17h664582d5029fc6c8E
	.p2align	4
_ZN3std2rt10lang_start17h664582d5029fc6c8E:
.seh_proc _ZN3std2rt10lang_start17h664582d5029fc6c8E
	sub	rsp, 56
	.seh_stackalloc 56
	.seh_endprologue
	mov	rax, r8
	mov	r8, rdx
	mov	qword ptr [rsp + 48], rcx
	mov	byte ptr [rsp + 32], r9b
	lea	rdx, [rip + anon.69f3f6443ce30bc560a04ff398775c09.0]
	lea	rcx, [rsp + 48]
	mov	r9, rax
	call	_RNvNtCs1aoCESei0z2_3std2rt19lang_start_internal
	nop
	.seh_startepilogue
	add	rsp, 56
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE,unique,1
	.p2align	4
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rcx, qword ptr [rcx]
	call	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E
	xor	eax, eax
	.seh_startepilogue
	add	rsp, 40
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E,unique,2
	.p2align	4
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	call	rcx
	nop
	#APP
	#NO_APP
	.seh_startepilogue
	add	rsp, 40
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hab7e21a6980e88dbE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hab7e21a6980e88dbE,unique,3
	.p2align	4
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hab7e21a6980e88dbE:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hab7e21a6980e88dbE
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	mov	rcx, qword ptr [rcx]
	call	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2e29a0141a1d6ec0E
	xor	eax, eax
	.seh_startepilogue
	add	rsp, 40
	.seh_endepilogue
	ret
	.seh_endproc

	.def	_ZN6r_hir24main17h5f03e7eb276d33a7E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN6r_hir24main17h5f03e7eb276d33a7E,unique,4
	.globl	_ZN6r_hir24main17h5f03e7eb276d33a7E
	.p2align	4
_ZN6r_hir24main17h5f03e7eb276d33a7E:
.seh_proc _ZN6r_hir24main17h5f03e7eb276d33a7E
	sub	rsp, 56
	.seh_stackalloc 56
	.seh_endprologue
	mov	dword ptr [rsp + 36], 11
	lea	rax, [rsp + 36]
	mov	qword ptr [rsp + 40], rax
	lea	rax, [rip + _RNvXs9_NtNtNtCs8icZUrvfcq9_4core3fmt3num3implNtB9_7Display3fmt]
	mov	qword ptr [rsp + 48], rax
	lea	rcx, [rip + anon.69f3f6443ce30bc560a04ff398775c09.1]
	lea	rdx, [rsp + 40]
	call	_RNvNtNtCs1aoCESei0z2_3std2io5stdio6__print
	nop
	.seh_startepilogue
	add	rsp, 56
	.seh_endepilogue
	ret
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main,unique,5
	.globl	main
	.p2align	4
main:
.seh_proc main
	sub	rsp, 56
	.seh_stackalloc 56
	.seh_endprologue
	mov	r9, rdx
	movsxd	r8, ecx
	lea	rax, [rip + _ZN6r_hir24main17h5f03e7eb276d33a7E]
	mov	qword ptr [rsp + 48], rax
	mov	byte ptr [rsp + 32], 0
	lea	rdx, [rip + anon.69f3f6443ce30bc560a04ff398775c09.0]
	lea	rcx, [rsp + 48]
	call	_RNvNtCs1aoCESei0z2_3std2rt19lang_start_internal
	nop
	.seh_startepilogue
	add	rsp, 56
	.seh_endepilogue
	ret
	.seh_endproc

	.section	.rdata,"dr",one_only,anon.69f3f6443ce30bc560a04ff398775c09.0,unique,6
	.p2align	3, 0x0
anon.69f3f6443ce30bc560a04ff398775c09.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hab7e21a6980e88dbE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h51489d1c7e35462fE

	.section	.rdata,"dr",one_only,anon.69f3f6443ce30bc560a04ff398775c09.1,unique,7
anon.69f3f6443ce30bc560a04ff398775c09.1:
	.asciz	"\004y = \300\001\n"
```

### All together

```
cargo +nightly rustc -- -Z unpretty=hir > hir.log
cargo +nightly rustc -- -Z unpretty=mir > mir.log
cargo rustc --release -- --emit=llvm-ir
cargo rustc --release -- --emit=asm -C "llvm-args=-x86-asm-syntax=intel"
```

### 🧭 Summary Table

| IR          | Stands for                       | Level    | Purpose                                      |
| ----------- | -------------------------------- | -------- | -------------------------------------------- |
| **HIR**     | High‑Level IR                    | High     | Desugared Rust, type‑checked, readable       |
| **THIR**    | Typed High‑Level IR              | High‑mid | Typed expression tree, used to build MIR     |
| **MIR**     | Mid‑Level IR                     | Mid      | Borrow checking, optimizations, control flow |
| **LLVM IR** | LLVM Intermediate Representation | Low      | Optimization + codegen backend               |



