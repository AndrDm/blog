---
title: Measuring the True Cost of x86 Div: 32‑bit vs 64‑bit with Rust and Inline Asm
date: 2026-04-28
description: Div
programming_languages:
  - Rust
  - Assembly
categories:
  - Programming
archives:
  - 2026-04
tags:
  - MyLearning
draft: false
---

Modern CPUs are fast—but some instructions still hide surprising costs. One of the most misunderstood is `DIV`. Is 32‑bit division faster than 64‑bit? Does instruction width matter anymore on x86‑64?

To answer this properly, we need more than wall‑clock timers. We need cycle counters, instruction retirement statistics, serialization barriers, and tight control over CPU affinity.

<!--more-->

In this post, we’ll build a **microbenchmark in Rust using inline assembly** that measures the real cost of integer division using hardware performance counters—and the results may challenge your assumptions.

### Why division benchmarks are tricky

Benchmarking arithmetic instructions on modern out‑of‑order CPUs is deceptively hard:

- CPUs reorder instructions aggressively
- Timers can be skewed by frequency scaling or migration across cores
- Modern CPUs overlap instruction execution, hiding latency
- The OS scheduler can move your thread mid‑measurement

If you want *instruction‑level truth*, you must:

1. Pin execution to a specific core
2. Serialize timestamp reads
3. Measure **cycles and retired instructions**, not time
4. Run enough repetitions to eliminate noise

This benchmark does exactly that.

## Pinning execution to a single core

We start by fixing the thread to a specific logical CPU using the Windows API:

```rust
fn pin_to_core(core: usize) {
    let mask: usize = 1usize << core;
    unsafe {
        let prev = SetThreadAffinityMask(GetCurrentThread(), mask);
        if prev == 0 {
            panic!("SetThreadAffinityMask failed for core {}", core);
        }
    }
}
```

Why this matters:

- Prevents scheduler migration
- Ensures invariant TSC behavior
- Keeps performance counters consistent

Without this, your measurements with rdpmc are garbage.

## High‑precision timing with RDTSC/RDTSCP and RDPMC

We use `RDTSC` and `RDTSCP` with fencing to get accurate cycle deltas, but at base CPU clock.

## Using hardware performance counters (RDPMC)

Clock cycles alone aren’t enough. We also want:

- **Unhalted core cycles**
- **Instructions retired**

```rust
#[inline(always)]
fn rdpmc(counter: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "lfence",
            "rdpmc",
            "lfence",
            out("eax") low,
            out("edx") high,
            in("ecx") counter,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}
```

We use:

```rust

const PMC_INSTRUCTIONS: u32 = 0x40000000;
const PMC_CYCLES: u32 = 0x40000001;

```

## The actual workload: forcing real division latency

To avoid dead‑code elimination or partial execution overlap, we repeat a real `div` instruction 1,000 times inside a single inline assembly block.

### 64‑bit division

```rust
#[inline(always)]
fn measured_div_64() {
    unsafe {
        asm!(
            ".rept 1000",
            "mov rcx, 0x2034",
            "mov rdx, 0x0008",
            "mov rax, 0x2B7C",
            "div rcx",
            ".endr",
            lateout("rax") _,
            lateout("rdx") _,
            lateout("rcx") _,
            options(nostack, nomem),
        );
    }
}

```

32‑bit division

```rust
#[inline(always)]
fn measured_div_32() {
    unsafe {
        asm!(
            ".rept 1000",
            "mov ecx, 0x2034",
            "mov edx, 0x0008",
            "mov eax, 0x2B7C",
            "div ecx",
            ".endr",
            lateout("rax") _,
            lateout("rdx") _,
            lateout("rcx") _,
            options(nostack, nomem),
        );
    }
}
```

## Full Code:

```rust
use std::arch::asm;
use windows::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};

fn pin_to_core(core: usize) {
    // Logical CPU → bitmask
    let mask: usize = 1usize << core;
    unsafe {
        let prev = SetThreadAffinityMask(GetCurrentThread(), mask);
        if prev == 0 {
            panic!("SetThreadAffinityMask failed for core {}", core);
        }
    }
}

#[inline(always)]
fn rdtsc_start() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
        "lfence",
        "rdtsc",
        out("eax") low,
        out("edx") high,
        options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
fn rdtscp_end() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
        "rdtscp",
        "lfence",
        out("eax") low,
        out("edx") high,
        out("ecx") _, // IA32_TSC_AUX (must be declared)
        options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
fn rdpmc(counter: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
        "lfence",
        "rdpmc",
        "lfence",
        out("eax") low,
        out("edx") high,
        in("ecx") counter,
        options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
fn measured_div_64() {
    unsafe {
        asm!(
        ".rept 1000",
        "mov rcx, 0x2034",
        "mov rdx, 0x0008",
        "mov rax, 0x2B7C",
        "div rcx",
        ".endr",
        lateout("rax") _, // div writes quotient
        lateout("rdx") _, // div writes remainder
        lateout("rcx") _, // rcx is modified by us
        options(nostack, nomem),
        );
    }
}

#[inline(always)]
fn measured_div_32() {
    unsafe {
        asm!(
        ".rept 1000",
        "mov ecx, 0x2034",
        "mov edx, 0x0008",
        "mov eax, 0x2B7C",
        "div ecx",
        ".endr",
        lateout("rax") _, // div writes quotient
        lateout("rdx") _, // div writes remainder
        lateout("rcx") _, // rcx is modified by us
        options(nostack, nomem),
        );
    }
}

fn get_core_from_args() -> usize {
    std::env::args()
        .nth(1) // first user argument
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)
}

fn main() {
    const ITER: usize = 1_000_000;
    const PMC_CYCLES: u32 = 0x40000001; // PMC1 Unhalted Core Cycles
    const PMC_INSTRUCTIONS: u32 = 0x40000000; // PMC0 Instructions Retired

    let mut min_ticks = u64::MAX;
    let mut min_cycles = u64::MAX;
    let mut min_instructions = u64::MAX;

    let core: usize = get_core_from_args();

    pin_to_core(core);

    println!("Running {} iterations on core {}", ITER, core);

    for _ in 0..ITER {
        let start = rdtsc_start();
        measured_div_64();
        let end = rdtscp_end();

        let delta = end.wrapping_sub(start);
        if delta < min_ticks {
            min_ticks = delta;
        }
    }

    for _ in 0..ITER {
        let start = rdpmc(PMC_CYCLES);
        measured_div_64();
        let end = rdpmc(PMC_CYCLES);

        let delta = end.wrapping_sub(start);
        if delta < min_cycles {
            min_cycles = delta;
        }
    }

    for _ in 0..ITER {
        let start = rdpmc(PMC_INSTRUCTIONS);
        measured_div_64();
        let end = rdpmc(PMC_INSTRUCTIONS);

        let delta = end.wrapping_sub(start);
        if delta < min_instructions {
            min_instructions = delta;
        }
    }

    let ipc = min_instructions as f64 / min_cycles as f64;
    println!(
        "64-bit registers: Ticks: {}, Cycles: {}, Instructions: {}, IPC: {:.2}",
        min_ticks, min_cycles, min_instructions, ipc
    );

    min_ticks = u64::MAX;
    min_cycles = u64::MAX;
    min_instructions = u64::MAX;

    for _ in 0..ITER {
        let start = rdtsc_start();
        measured_div_32();
        let end = rdtscp_end();

        let delta = end.wrapping_sub(start);
        if delta < min_ticks {
            min_ticks = delta;
        }
    }

    for _ in 0..ITER {
        let start = rdpmc(PMC_CYCLES);
        measured_div_32();
        let end = rdpmc(PMC_CYCLES);

        let delta = end.wrapping_sub(start);
        if delta < min_cycles {
            min_cycles = delta;
        }
    }

    for _ in 0..ITER {
        let start = rdpmc(PMC_INSTRUCTIONS);
        measured_div_32();
        let end = rdpmc(PMC_INSTRUCTIONS);

        let delta = end.wrapping_sub(start);
        if delta < min_instructions {
            min_instructions = delta;
        }
    }

    let ipc = min_instructions as f64 / min_cycles as f64;
    println!(
        "32-bit registers: Ticks: {}, Cycles: {}, Instructions: {}, IPC: {:.2}",
        min_ticks, min_cycles, min_instructions, ipc
    );
}
```

## Results

on i7-13850HX:

|                              | P Core 64-bit | P-Core 32 bit | E-Core 64-bit | E-Core 32bit |
| ---------------------------- | ------------- | ------------- | ------------- | ------------ |
| Ticks (rdtsc)                | 4534          | 2728          | 11538-11542   | 6688         |
| Instructions Retired         | 4007          | 4007          | 4008          | 4008         |
| CPU Cycles (rdpmc)           | 10057         | 6057          | 19073         | 11073        |
| IPC (Instructions Per Cycle) | 0,40          | 0,66          | 0,21          | 0,36         |

Div commented out:

```
        "mov ecx, 0x2034",
        "mov edx, 0x0008",
        "mov eax, 0x2B7C",
        // "div ecx",
```

IPC now:

|               | P Core 64-bit | P-Core 32 bit | E-Core 64-bit | E-Core 32bit |
| ------------- | ------------- | ------------- | ------------- | ------------ |
| Ticks (rdtsc) | 264           | 288           | 480           | 470          |
| Instructions  | 3007          | 3007          | 3008          | 3008         |
| CPU Cycles    | 694-695       | 648           | 843-844       | 820          |
| IPC           | 4,33          | 4,64          | 3,56-3,57     | 3,67         |

