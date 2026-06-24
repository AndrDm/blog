## Zero-Cost Abstractions — A Practical Check

I have seen many posts about *“zero-cost abstractions.”* Let’s look at this more carefully — this makes for a good technical exercise.

### Test Setup

Tested with:

- **Rust 1.96.0** (released May 28, 2026)
- **GCC 16.1.0** (released April 30, 2026)

### Source Code

Rust

```rust
use std::time::Instant;

fn main() {
    // Create 2_147_418_111 integers to get 65536 sum
    let data: Vec<i32> = (0..2_147_418_111).collect();

    // Warm-up
    let _warmup: i32 = data.iter().filter(|x| **x % 2 == 0).map(|x| x * 2).sum();

    // Measure iterator chain
    let start = Instant::now();
    let sum_iter: i32 = data.iter().filter(|x| **x % 2 == 0).map(|x| x * 2).sum();
    let dur_iter = start.elapsed();

    // Measure equivalent for-loop
    let start = Instant::now();
    let mut sum_loop = 0i32;
    for x in &data {
        if x % 2 == 0 {
            sum_loop += x * 2;
        }
    }
    let dur_loop = start.elapsed();

    println!("Iterator sum: {}; time: {:?}", sum_iter, dur_iter);
    println!("Loop sum: {}; time: {:?}", sum_loop, dur_loop);
}

```

C++

```cpp
#include <vector>
#include <numeric>
#include <chrono>
#include <fmt/core.h>

using namespace std;
using fmt::print;

int main() {
    vector<int> data;
	data.reserve(2'147'418'111); // Same amount as in Rust

	for (int i = 0; i < data.capacity(); ++i) {
    	data.push_back(i);
	}
    volatile int warmup = accumulate(
        data.begin(), data.end(), 0,
        [](int acc, int x) {
            return (x % 2 == 0) ? acc + x * 2 : acc;
        }
    );

    auto start = chrono::high_resolution_clock::now();
    int sum_iter = accumulate(
        data.begin(), data.end(), 0,
        [](int acc, int x) {
            return (x % 2 == 0) ? acc + x * 2 : acc;
        }
    );
    auto dur_iter = chrono::high_resolution_clock::now() - start;

    start = chrono::high_resolution_clock::now();
    int sum_loop = 0;
    for (int x : data) {
        if (x % 2 == 0) {
            sum_loop += x * 2;
        }
    }
    auto dur_loop = chrono::high_resolution_clock::now() - start;

	print("Iterator sum: {}; time: {}ms\n",
    	sum_iter,
    	chrono::duration<double, milli>(dur_iter).count());

	print("Loop sum: {}; time: {}ms\n",
    	sum_loop,
    	chrono::duration<double, milli>(dur_loop).count());
    return 0;
}
```

As you can see the Rust code is cleaner and shorter.

### Results

On Xeon w5-2445 @ 3.1 GHz:

|      | O3/Release | with =native |
| ---- | ---------- | ------------ |
| Rust | 713-714 ms | 571-572 ms   |
| C++  | 751-752 ms | 654-655 ms   |

Rust Release:

```
>r-bench.exe
Iterator sum: 65536; time: 713.4694 ms
Loop sum: 65536; time: 713.5939 ms
```

Rust with rustflags = ["-C", "target-cpu=native"]:

```
>r-perf.exe
Iterator sum: 65536; time: 571.5756ms
Loop sum: 65536; time: 571.1088ms
```

C++ O3:

```
g++ -O3 -std=c++20 bench.cpp -o bench -lfmt
>bench.exe
Iterator sum: 65536; time: 751.3733 ms
Loop sum: 65536; time: 751.3685 ms
```

C++ with -march=native:

```
g++ -O3 -march=native -std=c++20 bench.cpp -o bench -lfmt
>c-bench.exe
Iterator sum: 65536; time: 655.0348 ms
Loop sum: 65536; time: 654.454 ms
```

### Observations

- In Rust, the iterator pipeline and the explicit `for` loop produce **identical performance**.
- In C++, the same holds: `std::accumulate` and a manual loop perform equivalently.
- C++ is slightly slower in this test.

### Assembly Analysis

To verify what is happening, we inspect the generated assembly.

Rust:

```
cargo rustc --release -- --emit asm -C opt-level=3 -C  "llvm-args=-x86-asm-syntax=intel"
```

GCC:

```
g++ -O3 -march=native -std=c++20 -S -fverbose-asm -masm=intel bench.cpp
```

Or, alternatively, get it from built executable with objdump

```
objdump -d -M intel bench.exe > dump.s
```

Hot Loops:

```rust
let sum_iter: i32 = data.iter().filter(|x| **x % 2 == 0).map(|x| x * 2).sum();
// and
    for x in &data {
        if x % 2 == 0 {
            sum_loop += x * 2;
        }
    }
```

Both compile into a SIMD loop:

```
.LBB4_8:
	movdqu	xmm3, xmmword ptr [r9 + 4*rax - 16]
	movdqu	xmm4, xmmword ptr [r9 + 4*rax]
	movdqa	xmm5, xmm3
	pand	xmm5, xmm6
	movdqa	xmm7, xmm4
	pand	xmm7, xmm6
	paddd	xmm3, xmm3
	paddd	xmm4, xmm4
	pcmpeqd	xmm5, xmm0
	pand	xmm5, xmm3
	paddd	xmm2, xmm5
	pcmpeqd	xmm7, xmm0
	pand	xmm7, xmm4
	paddd	xmm1, xmm7
	add	rax, 8
	cmp	rax, 2147418108
	jne	.LBB4_8
```

And in case of C++

```cpp
    int sum_iter = accumulate(
        data.begin(), data.end(), 0,
        [](int acc, int x) {
            return (x % 2 == 0) ? acc + x * 2 : acc;
        }
    );
// and
    for (int x : data) {
        if (x % 2 == 0) {
            sum_loop += x * 2;
        }
    }
```

Both compiled to 

```
.L39:
	vmovdqu	ymm0, YMMWORD PTR [rsi]
	add	rsi, 32
	vpandd	ymm2, ymm0, ymm3
	vpslld	ymm0, ymm0, 1
	vptestnmd	k1, ymm2, ymm2
	vpaddd	ymm1{k1}, ymm1, ymm0
	cmp	rax, rsi
	jne	.L39
```

### Why is C++ slightly faster?

The difference comes from vectorization:

- Rust emits **128-bit SIMD (xmm)** instructions
- GCC emits **256-bit SIMD (ymm)** instructions

👉 Wider SIMD registers allow processing more elements per iteration, resulting in slightly better throughput.