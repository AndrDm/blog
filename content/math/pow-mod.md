---
title: Finding the Remainder of \(9^{2018} \mod 7\) - Understanding Modular Patterns
date: 2026-05-20
description: CPU String to LabVIEW
programming_languages:
  - Rust
categories:
  - Programming
archives:
  - 2026-05
tags:
  - Math
draft: false
---

When you first see an expression like  


\[
9^{2018} \mod 7,
\]


it looks impossible to compute. The number \(9^{2018}\) is unimaginably large — far beyond what any calculator can display.

Fortunately, modular arithmetic has a wonderful property: **even enormous powers often fall into small repeating cycles**. Once you notice the pattern, the problem becomes surprisingly simple.

This post walks you through the reasoning step by step.

<!--more-->

## 1. Reduce the Base First

Before doing anything complicated, reduce the base modulo 7:



\[
9 \mod 7 = 2.
\]



So the original expression becomes:



\[
9^{2018} \mod 7 = 2^{2018} \mod 7.
\]



Now the base is small, but the exponent is still huge. Time to look for structure.

---

## 2. Explore the Pattern of Powers

Let’s compute the first few powers of 2 modulo 7:

| \(k\) | \(2^k \mod 7\) |
|------|----------------|
| 1    | 2              |
| 2    | 4              |
| 3    | 1              |
| 4    | 2              |
| 5    | 4              |
| 6    | 1              |

A clear cycle appears:



\[
2,\; 4,\; 1,\; 2,\; 4,\; 1,\; \dots
\]



The pattern repeats every **3 steps**.  
This repeating block is called the **period**:



\[
T = 3.
\]



---

## 3. Reduce the Exponent Using the Cycle

Since the cycle length is 3, we only need:



\[
2018 \mod 3.
\]



Compute it:



\[
2018 = 3 \cdot 672 + 2.
\]



So:



\[
2^{2018} \mod 7 = 2^{2} \mod 7.
\]



And:



\[
2^2 = 4.
\]



---

## ✔ Final Answer



\[
\boxed{4}
\]



---

## 4. A Faster Method: Fermat’s Little Theorem

There’s an even shorter way to reach the same result.

Fermat’s Little Theorem states:



\[
a^{p-1} \equiv 1 \pmod{p}
\]


for any integer \(a\) not divisible by a prime \(p\).

Here, \(p = 7\).  
So:



\[
9^{6} \equiv 1 \pmod{7}.
\]



Now reduce both the base and the exponent:

- \(9 \mod 7 = 2\)
- \(2018 \mod 6 = 2\)

Thus:



\[
9^{2018} \mod 7 = 2^{2} \mod 7 = 4.
\]



Same result, less work.

---

## Why This Works

Problems like this appear in:

- number theory  
- cryptography  
- competitive programming  
- algorithm design  

Recognizing modular cycles is a powerful technique. It transforms intimidating expressions into simple arithmetic.

---

Basic Rust code:

```rust
use malachite_base::num::arithmetic::traits::ModPow;
use malachite_nz::natural::Natural;

fn main() {
    let mut base = Natural::from(9u32);
    let exp = Natural::from(2018u32);
    let modulus = Natural::from(7u32);

    base = base % &modulus;
    let result = base.mod_pow(&exp, &modulus);
    println!("{}", result);
}
```

Cargo.toml:

```toml
[package]
name = "r-big-num-mod"
version = "0.1.0"
edition = "2024"

[dependencies]
malachite-base = "0.9"
malachite-nz = "0.9"
malachite-q = "0.9"
```

Output:

```
>cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\r-big-num-mod.exe`
4
```
 
