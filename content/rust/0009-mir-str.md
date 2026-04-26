---
title: How Rust’s Borrow Checker Tracks “Alive” and “Dead” References (String Example with HIR & MIR)
date: 2026-04-26
description: HIR MIR
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

One of the most common early borrow‑checker surprises in Rust happens when working with `String` and references. Developers often expect a borrow to end at the line where it is created — but Rust extends the borrow until the **last use** of the reference.

<!--more-->

To understand why, we’ll walk through a classic example and examine how Rust interprets it using **HIR** (High‑level IR) and **MIR** (Mid‑level IR). MIR is especially important because this is where the borrow checker performs its analysis.

# **The Classic Example**

rust

```
fn main() {
    let mut s = String::from("hello");
    let r = &s;
    println!("{}", r);
    println!("{}", s); // error if placed before r is last used
}
```

If you move the last `println!` above the use of `r`, Rust will complain:

Code

```
cannot borrow `s` as immutable because it is also borrowed as immutable
```

Let’s see why.

# **HIR: The High‑Level Structure**

HIR is a cleaned‑up version of your code. It still looks like Rust, but with syntactic sugar removed.

A simplified HIR for this example:

Code

```rust
fn main() {
    let mut s: String = String::from("hello");
    let r: &String = &s;
    println!("{}", r);
    println!("{}", s);
}
```

HIR defines:

- where references are created
- their scopes
- the order of evaluation

But the borrow checker does **not** operate on HIR — it operates on MIR.

# **MIR: Where Borrow Checking Actually Happens**

Here is a simplified MIR representation of the function:

Code

```
bb0: {
    _1 = String::from("hello");   // s
    _2 = &_1;                     // r = &s            <-- borrow begins
    _3 = println!("{}", _2);      // println!(r)       <-- borrow still active
    _4 = println!("{}", _1);      // println!(s)       <-- illegal use if above
    return;
}
```

Now let’s annotate it with borrow‑checker semantics.

# **Where the Borrow Checker Sees the Reference as “Alive”**

Code

```
_2 = &_1;                         // ← immutable borrow begins
_3 = println!("{}", _2);          // ← reference still alive
_4 = println!("{}", _1);          // ← ERROR if placed before _3
```

The rule is simple but strict:

> **A borrow lives until the last use of the reference.**

So in this example:

- The borrow of `s` starts when `r = &s` is created.
- The borrow ends **after** the last use of `r` in `println!("{}", r)`.

If you try to use `s` before that last use, Rust rejects the code.

# **Why the Borrow Lives Longer Than Expected**

Many developers assume:

> “The borrow ends at the end of the line where I created the reference.”

But Rust’s actual rule is:

> **The borrow ends when the reference is no longer used.**

This is why the following fails:

rust

```
let r = &s;
println!("{}", s); // error
println!("{}", r);
```

Even though the reference was created earlier, it is still considered **alive** until its last use.

# **How to Make the Borrow End Earlier**

You can explicitly limit the lifetime of the reference by using a block:

rust

```
{
    let r = &s;
    println!("{}", r);
} // ← borrow ends here

println!("{}", s); // now allowed
```

This creates a separate MIR region, and the borrow checker can clearly see that `r` is dead before the final `println!`.

# **Conclusion**

Rust’s borrow checker is not arbitrary — it is following precise rules based on MIR, not the surface syntax of your code. The key idea is:

> **A reference stays alive until its last use, not until the line where it was created.**

By understanding how Rust lowers your code into HIR and MIR, you can predict exactly when a borrow begins and ends — and avoid confusing borrow‑checker errors.

### Built with

```
cargo +nightly rustc -- -Zunpretty=hir > hir.log
cargo +nightly rustc -- -Zunpretty=mir > mir.log
```

### Addon

# ✅ **Version 1: Mutate the String Before Borrowing**

rust

```
fn main() {
    let mut s = String::from("hello");
    s.push('!');                  // mutation → no warning

    let r = &s;
    println!("{}", r);
    println!("{}", s);
}
```

This keeps the example simple and avoids the warning.

# ✅ **Version 2: Mutate After the Immutable Borrow Ends**

This version demonstrates borrow‑checker behavior *and* uses `mut` legitimately:

rust

```
fn main() {
    let mut s = String::from("hello");

    {
        let r = &s;
        println!("{}", r);       // last use of r
    } // r dies here

    s.push_str(" world");        // mutation → allowed
    println!("{}", s);
}
```

This is a great example for a blog because it shows:

- immutable borrow
- borrow ending early due to block
- later mutation

# ✅ **Version 3: Mutate Using a Mutable Reference**

This version shows both immutable and mutable borrows:

rust

```
fn main() {
    let mut s = String::from("hello");

    let r = &s;
    println!("{}", r);           // last use of r

    let m = &mut s;              // now allowed
    m.push_str(" world");        // mutation
    println!("{}", m);
}
```

This demonstrates:

- immutable borrow (`r`)
- borrow ends at last use
- mutable borrow (`m`)
- mutation through mutable reference
