---
title: 10 Ways to Add Two Large Arrays in Rust Using ndarray
date: 2026-04-10
authorbox: false
sidebar: false
description: This article walks through ten different techniques for adding two large Array2<f32> matrices in Rust using ndarray
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
Rust’s ndarray crate is a powerful foundation for numerical and scientific computing — but just like with any data‑heavy workload, performance depends heavily on how you write your loops.
To explore this, I built a small educational demo that evaluates 10 different techniques for adding two 2D matrices (Array2<f32>)
<!--more-->
ranging from:

✅ Plain nested loops
✅ Slice‑based linear iteration
✅ indexed_iter_mut()
✅ Zip::for_each
✅ Built‑in ndarray operators
✅ In‑place operations
✅ Unsafe unchecked indexing
✅ SIMD (AVX2)
✅ SIMD + Rayon parallelism (fastest)

Each method is benchmarked on a 1024×1024 or 4096×4096 array depending on build mode.

<iframe src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b911bfbad5be311386378aa1957bbe43" width="1200" height="1000" allowfullscreen frameBorder="0" scrolling="no"></iframe>

Github: https://github.com/AndrDm/r-ndarray

<details>
  <summary>Code Snippet</summary>

```toml
[package]
name = "r-arr2"
version = "0.1.0"
edition = "2024"

[dependencies]
ndarray = { version = "0.17.2", features = ["rayon"] }
rayon = "1.11.0"
wide = "1.2.0"
```

```rust
//===================================================================================
//
// Title:        ndarray Array Addition Examples (Educational Demo)
// Purpose:      Demonstrate multiple ways to add two 2D Array2<f32> matrices
//               in Rust using ndarray, including:
//                  ✅ Plain nested loops
//                  ✅ Slice-based linear iteration
//                  ✅ indexed_iter_mut()
//                  ✅ Zip (sequential and parallel)
//                  ✅ Built‑in ndarray operators
//                  ✅ In‑place arithmetic
//                  ✅ Unsafe unchecked indexing
//                  ✅ SIMD using wide::f32x8 + Parallel version
//
// Created on:   09.04.2026 at 08:45:15 by AD.
//
//===================================================================================
use ndarray::{Array2, Zip};
use rayon::{
	// (parallel slice + iterator traits)
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::{ParallelSlice, ParallelSliceMut},
};
use std::time::Instant; // From Standard library
use wide::f32x8; // for SIMD

fn main() {
	// 1024x1024 = 1.048.576 elements (4 MiB); 4096x4096 = 16.777.216 elts (64 MiB)
	const ROWS: usize = if cfg!(debug_assertions) { 1024 } else { 4096 };
	const COLS: usize = if cfg!(debug_assertions) { 1024 } else { 4096 };

	println!("Hello, world of ndarray ({}x{})!", ROWS, COLS);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));
	// res = a + b;
	let t1 = Instant::now();
	//===============================================================================
	// 1) Simple, explicit nested for loops (row/column loops)
	// This version is closest to what you'd write in C — very easy.
	// ✅ Easiest to read
	// ✅ Clear memory indexing
	// ❌ Average performance and least idiomatic
	//
	for i in 0..ROWS {
		for j in 0..COLS {
			res[(i, j)] = a[(i, j)] + b[(i, j)];
		}
	}

	println!(
		"Done 1  (plain loops)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t1.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);

	let t2 = Instant::now();
	//============================================================================I==
	// 2) Using built‑in ndarray element‑wise addition
	// If the goal is “just add arrays”, this is the canonical one-liner.
	// ✅ Very short
	// ✅ Uses ndarray’s operator overloading
	// ❌ Creates a new array (cannot reuse res)
	// ❌ Not good when you need in-place updates
	//
	let res = &a + &b;

	println!(
		"Done 2  (built-in)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t2.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t3 = Instant::now();
	//===============================================================================
	// 3) Single for loop using .as_slice().unwrap()
	// This flattens the arrays and performs element wise addition with one loop.
	// ✅ One loop only
	// ✅ Very fast — linear memory access
	// ❌ Requires contiguous arrays (Array2 from from_elem is contiguous it's fine)
	//
	let a_slice = a.as_slice().unwrap();
	let b_slice = b.as_slice().unwrap();
	let res_slice = res.as_slice_mut().unwrap();

	for i in 0..a_slice.len() {
		res_slice[i] = a_slice[i] + b_slice[i];
	}

	println!(
		"Done 3  (slices)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t3.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t4 = Instant::now();
	//===============================================================================
	// 4) Using .indexed_iter_mut() (clean, explicit, safe)
	// ✅ Cleaner than nested loops — no manual outer/inner loops
	// ✅ Still very explicit: you see the indices
	// ✅ Works regardless of array memory layout (unlike .as_slice())
	// ✅ More Rust‑idiomatic than raw index loops
	// ❌ Slightly slower than Zip
	// Because each iteration does two bounds‑checked a[(i, j)] lookups.
	// (You can remove bounds checks by using uget() if desired.)
	//
	for ((i, j), r) in res.indexed_iter_mut() {
		*r = a[(i, j)] + b[(i, j)];
	}

	println!(
		"Done 4  (indexed_iter)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t4.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t5a = Instant::now();
	//==========================================================================P=I==
	// 5A) Idiomatic Rust using ndarray::Zip (recommended!)
	// This is the cleanest and most idiomatic way in ndarray.
	// ✅ Most idiomatic
	// ✅ Auto‑vectorization friendly
	// ✅ Handles broadcasting
	// ✅ No manual indexing
	//
	Zip::from(&mut res).and(&a).and(&b).for_each(|r, &x, &y| {
		*r = x + y;
	});

	println!(
		"Done 5A (for_each)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t5a.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t5b = Instant::now();
	//==========================================================================P=I==
	// 5B) Idiomatic Rust using ndarray::Zip Zip with broadcasting example
	//

	Zip::from(&mut res).and(&a).and(&b).for_each(|r, &x, &y| *r = x + y);

	println!(
		"Done 5B (for_each)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t5b.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);

	let t6a = Instant::now();
	//============================================================================I==
	// 6A) In‑place addition using +=
	// ✅ Very idiomatic
	// ✅ Zero manual loops
	// ✅ Leverages ndarray’s broadcasting + in-place math
	// ✅ Usually fast
	// ❌ Requires ownership over res
	//
	let mut res = a.clone();
	res += &b;

	println!(
		"Done 6A (inplace +=)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t6a.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t6b = Instant::now();
	//============================================================================I==
	// 6B) In‑place addition using += without clone
	//
	res += &a;
	res += &b;

	println!(
		"Done 6B (inplace +=)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t6b.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t7 = Instant::now();
	//==========================================================================P====
	// 7) Parallel version (educational for parallelism)
	// ✅ Shows how to parallelize data‑parallel loops
	// ✅ Very appealing for performance education
	// ❌ Requires enabling features = ["rayon"] in Cargo.toml:
	// ndarray = { version = "0.17.2", features = ["rayon"] }
	// When the rayon feature is enabled, ndarray does add parallel iterator traits
	// Do not need to import the parallel prelude explicitly when using par_for_each
	// ❌ Does NOT use explicit SIMD, but compiler may autovectorize
	//
	Zip::from(&mut res).and(&a).and(&b).par_for_each(|r, &x, &y| {
		*r = x + y;
	});

	println!(
		"Done 7  (par_for_each)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t7.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t8 = Instant::now();
	//==========================================================================P====
	// 8) Unsafe fast version using unchecked indexing
	// ✅ Shows how to skip bounds checks
	// ✅ Good for “performance tuning” lessons
	// ❌ Not recommended for beginners
	// ⚠️ unsafe!
	//
	for ((i, j), r) in res.indexed_iter_mut() {
		unsafe {
			*r = *a.uget([i, j]) + *b.uget([i, j]);
		}
	}

	println!(
		"Done 8  (unsafe)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t8.elapsed().as_millis()
	);

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t9 = Instant::now();
	//===============================================================================
	// 9) SIMD Version using wide::f32x8 (AVX2) and not std::simd
	// ✅ Fastest scalar-friendly method (explicit SIMD)
	// ✅ Uses portable vectors on stable Rust
	// ✅ Processes 8 elements per iteration
	// ❌ Requires arrays to be contiguous (they are here)
	// ℹ️ Note: You can change LANES to 16 on CPUs supporting AVX512.
	//
	const LANES: usize = 8;

	let a_slice = a.as_slice().unwrap();
	let b_slice = b.as_slice().unwrap();
	let res_slice = res.as_slice_mut().unwrap();

	let len = a_slice.len();
	let width = LANES;
	let chunks = len / width;

	for chunk in 0..chunks {
		let idx = chunk * width;

		// Load 8 floats into arrays
		let va = f32x8::new(a_slice[idx..idx + LANES].try_into().unwrap());
		let vb = f32x8::new(b_slice[idx..idx + LANES].try_into().unwrap());

		let vr = va + vb; // ADD

		// Store result back
		let arr = vr.to_array();
		res_slice[idx..idx + width].copy_from_slice(&arr);
	}

	// Scalar tail
	for i in (chunks * width)..len {
		res_slice[i] = a_slice[i] + b_slice[i];
	}

	println!(
		"Done 9  (SIMD AVX2)\t- res[0][0] = {}, time = {} ms",
		res[(0, 0)],
		t9.elapsed().as_millis()
	);

	//===============================================================================
	// ✅ Option: helper function (tiny and clean)
	#[inline]
	fn load8(slice: &[f32], idx: usize) -> f32x8 {
		f32x8::new(slice[idx..idx + 8].try_into().unwrap())
	}

	//===============================================================================
	//✅ Option: a neat macro (closest to real SIMD “load” semantics)
	macro_rules! load8 {
		($slice:expr, $i:expr) => {
			f32x8::new($slice[$i..$i + 8].try_into().unwrap())
		};
	}

	let a = Array2::<f32>::from_elem((ROWS, COLS), 1.0);
	let b = Array2::<f32>::from_elem((ROWS, COLS), 2.0);
	let mut res = Array2::<f32>::zeros((ROWS, COLS));

	let t10 = Instant::now();
	//===============================================================================
	// 10) SIMD + Parallel version (SIMD on all CPU threads)
	// ✅ Uses Rayon to split work across CPU cores
	// ✅ Uses wide::f32x8 SIMD inside each thread
	// ✅ Fastest version for large arrays
	// ✅ Perfect for demonstrating hybrid data-parallel + SIMD parallelism
	// ❌ Requires arrays to be contiguous
	//

	let a_slice = a.as_slice().unwrap();
	let b_slice = b.as_slice().unwrap();
	let res_slice = res.as_slice_mut().unwrap();
	type RowChunk<'a> = (&'a mut [f32], &'a [f32]);
	res_slice
		.par_chunks_mut(COLS)
		.zip(a_slice.par_chunks(COLS))
		.zip(b_slice.par_chunks(COLS))
		.for_each(|((res_row, a_row), b_row): (RowChunk, &[f32])| {
			let row_len = a_row.len();
			let chunks = row_len / LANES;

			// SIMD part
			for c in 0..chunks {
				let idx = c * LANES;

				let va = load8(a_row, idx); // Usage of fn
				let vb = load8!(b_row, idx); // Usage of macro
				let vr = va + vb;

				let result = vr.to_array();
				res_row[idx..idx + LANES].copy_from_slice(&result);
			}

			// Scalar tail
			for i in (chunks * LANES)..row_len {
				res_row[i] = a_row[i] + b_row[i];
			}
		});

	println!(
		"Done 10 (SIMD/Parallel)\t- res[0][0] = {}, time = {} ms",
		res_slice[0],
		t10.elapsed().as_millis()
	);
}
  ```

</details>

