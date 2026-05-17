---
title: Getting CPU Base Frequency (code snippet)
date: 2026-05-16
description: CPU String to LabVIEW
programming_languages:
  - Rust
  - LabVIEW
  - C
categories:
  - Programming
archives:
  - 2026-05
tags:
  - CPU
draft: false
---

A simple exercise — how to obtain the CPU base frequency.

<!--more-->

Basic Rust code:

```rust
use std::arch::x86_64::_rdtsc;
use windows::Win32::System::Performance::{
    QueryPerformanceCounter, QueryPerformanceFrequency,
};

fn main() {
    unsafe {
        let mut freq: i64 = 0;
        QueryPerformanceFrequency(&mut freq).expect("QPF failed");

        println!("Performance counter frequency: {} Hz", freq);

        loop {
            let mut start: i64 = 0;
            QueryPerformanceCounter(&mut start).expect("QPC failed");

            let tsc_start = _rdtsc();

            loop {
                let mut now: i64 = 0;
                QueryPerformanceCounter(&mut now).expect("QPC failed");
                if now - start >= freq {
                    break;
                }
            }

            let tsc_end = _rdtsc();
            let delta = tsc_end - tsc_start;
            let mhz = delta as f64 / 1_000_000.0;

            println!("Measured CPU frequency: {:.5} MHz", mhz);
        }
    }
}
```

Cargo.toml:

```toml
[package]
name = "r-cpu_freq"
version = "0.1.0"
edition = "2024"


[dependencies]
windows = { version = "0.62", features = [
    "Win32_System_Performance",
] }
```

Output:

```
>cargo run
   Compiling r-cpu_freq v0.1.0 (C:\Users\Andrey\Desktop\r-cpu_freq)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `C:\Users\Andrey\Desktop\r-cpu_freq\target\debug\r-cpu_freq.exe`
Performance counter frequency: 10000000 Hz
Measured CPU frequency: 2693.76260 MHz
Measured CPU frequency: 2693.76255 MHz
Measured CPU frequency: 2693.76273 MHz
```

Same C Code

```cpp
#include <windows.h>
#include <stdint.h>
#include <stdio.h>

static inline uint64_t rdtsc() {
    return __rdtsc();
}

int main() {
    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);

    printf("Performance counter frequency: %lld Hz\n", freq.QuadPart);

    while (true) {
        LARGE_INTEGER start, now;
        QueryPerformanceCounter(&start);

        uint64_t tsc_start = rdtsc();

        // Poll until 1 second has passed
        do {
            QueryPerformanceCounter(&now);
        } while ((now.QuadPart - start.QuadPart) < freq.QuadPart);

        uint64_t tsc_end = rdtsc();
        uint64_t delta = tsc_end - tsc_start;

        double cpu_mhz = (double)delta / 1e6;

        printf("Measured CPU frequency: %.2f MHz\n", cpu_mhz);
        Sleep(500);
    }

    return 0;
}
```

NI CVI Code

```c
#include <windows.h>
#include <ansi_c.h>

uint64_t rdtsc(void)
{
    uint32_t lo=0, hi=0;

    __asm {
        rdtsc
        mov lo, eax
        mov hi, edx
    }

    return ((uint64_t)hi << 32) | lo;
}

int main() {
    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);

    printf("Performance counter frequency: %lld Hz\n", freq.QuadPart);

    while (1) {
        LARGE_INTEGER start, now;
        QueryPerformanceCounter(&start);

        uint64_t tsc_start = rdtsc();

        // Poll until 1 second has passed
        do {
            QueryPerformanceCounter(&now);
        } while ((now.QuadPart - start.QuadPart) < freq.QuadPart);

        uint64_t tsc_end = rdtsc();
        uint64_t delta = tsc_end - tsc_start;

        double cpu_mhz = (double)delta / 1e6;

        printf("Measured CPU frequency: %.2f MHz\n", cpu_mhz);
        Sleep(500);
    }
}
```

