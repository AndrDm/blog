---
title: Understanding Traits in Rust — Building a Chain Writer Pipeline
date: 2026-08-29
description: A practical exploration of Rust’s Write trait through custom decorators
programming_languages:
  - Rust
categories:
  - Programming
archives:
  - 2026-08
tags:
  - Rust
  - Trait
draft: false
---

Traits are one of Rust’s most important building blocks. They define **capabilities**, not **types**. A trait says: *“Any type that implements these methods can be used here.”*. Rust’s `Write` trait is one of those quiet workhorses that becomes incredibly powerful once you start wrapping and composing writers. In this post, I’ll walk through a small but surprisingly flexible pattern: **stackable writer decorators**.

<!--more-->

The idea is simple:

- one writer **normalizes line endings** (`\r` → `\n`)
- another writer **uppercases all bytes**
- a third writer **tees** output to multiple destinations (stdout + file)

Each decorator implements `Write`, so they can be nested freely. The result is a pipeline that transforms and routes data as it flows through.

```rust
use std::fs::File;
use std::io::{stdout, Write, Result};

//
// NormalizeEol: converts '\r' → '\n' on write
//
struct NormalizeEol<W: Write> {
    inner: W,
}

impl<W: Write> Write for NormalizeEol<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let converted: Vec<u8> = buf
            .iter()
            .map(|&b| if b == b'\r' { b'\n' } else { b })
            .collect();

        self.inner.write(&converted)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

//
// UppercaseWriter: converts all bytes to ASCII uppercase
//
struct UppercaseWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for UppercaseWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let upper: Vec<u8> = buf.iter().map(|b| b.to_ascii_uppercase()).collect();
        self.inner.write(&upper)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

//
// TeeWriter: writes to two writers (like Unix 'tee')
//
struct TeeWriter<A: Write, B: Write> {
    a: A,
    b: B,
}

impl<A: Write, B: Write> Write for TeeWriter<A, B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.a.write_all(buf)?;
        self.b.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.a.flush()?;
        self.b.flush()
    }
}

//
// MAIN: chain decorators → NormalizeEol → Uppercase → Tee → File
//
fn main() -> Result<()> {
    // Final destination: file
    let file = File::create("out.txt")?;

    // Tee: file + stdout
    let tee = TeeWriter {
        a: file,
        b: stdout(),
    };

	/*
    // Uppercase layer
    let upper = UppercaseWriter { inner: tee }; // move

    // Normalize EOL layer (top of chain)
    let mut writer = NormalizeEol { inner: upper };
    */

    // Or singleliner
	let mut writer = NormalizeEol { inner: UppercaseWriter { inner: tee } };

    // Write text containing CR characters
    writer.write_all(b"Hello\rWorld\r")?;
    writer.flush()?;

    Ok(())
}
```

## **Why this is elegant**

- Each decorator is **independent**.
- They compose naturally because they all implement `Write`.
- You can reorder them, remove them, or add new ones.
- The single‑line nested struct literal is surprisingly readable once you get used to it.

This pattern scales beautifully — compression filters, encryption layers, logging taps, metrics collectors, throttlers, formatters, you name it.

### **1. Polymorphism without inheritance**

In languages like Java or C++, you’d create a base class:

Code

```
class Writer { virtual void write(...) = 0; }
```

Rust avoids inheritance entirely. Instead, you implement a trait:

rust

```
impl<W: Write> Write for NormalizeEol<W> { ... }
```

This means:

- `NormalizeEol<File>` is a writer
- `NormalizeEol<TeeWriter<File, Stdout>>` is a writer
- `NormalizeEol<UppercaseWriter<TeeWriter<File, Stdout>>>` is a writer

You get polymorphism **without** the complexity of class hierarchies.

### **2. Zero‑cost abstraction**

Traits compile down to extremely efficient machine code.

Your decorator chain:

Code

```
NormalizeEol → UppercaseWriter → TeeWriter → File
```

is resolved at compile time. Rust knows the exact types involved — no dynamic dispatch unless you explicitly ask for it.

This means:

- no heap allocation
- no runtime lookup
- no virtual function overhead
- no trait object unless you choose one

It’s all **static dispatch**, inlined aggressively.

### **3. Composability through generics**

Your decorators are generic:

rust

```
struct UppercaseWriter<W: Write> {
    inner: W,
}
```

This is the magic.

You’re not writing:

Code

```
UppercaseWriter<File>
UppercaseWriter<TcpStream>
UppercaseWriter<Vec<u8>>
```

You’re writing:

Code

```
UppercaseWriter<W>
```

which works for **any** writer.

This is the same pattern used in:

- `BufWriter<W>`
- `GzEncoder<W>`
- `DeflateEncoder<W>`
- `LineWriter<W>`
- `TeeReader<R>`
- `Cursor<Vec<u8>>`

Your code is following the exact design philosophy of Rust’s standard library.

## 🧱 Why Your Writer Decorators Are a Perfect Trait Demo

Your example is small, but it demonstrates **every important trait concept**:

### ✔ Trait implementation

Each struct implements `Write`.

### ✔ Generic type parameters

Each decorator wraps *any* writer.

### ✔ Trait bounds

`W: Write` ensures the inner type supports writing.

### ✔ Static dispatch

The compiler knows the full chain at compile time.

### ✔ Composition

You build a pipeline by nesting writers.

### ✔ Zero‑cost abstraction

No runtime overhead — just clean transformations.

### ✔ Extensibility

You can add new decorators without touching existing ones.

This is exactly how professional Rust libraries build:

- compression stacks
- encryption layers
- logging taps
- protocol filters
- metrics collectors
- format normalizers
- network throttlers

Your example is small, but it’s **idiomatic Rust** at its finest.

## 🔧 Trait Objects (Optional Deep Dive)

Your current design uses **static dispatch**, which is ideal.

But Rust also allows **dynamic dispatch** via trait objects:

rust

```
let writer: Box<dyn Write> = Box::new(
    NormalizeEol {
        inner: UppercaseWriter {
            inner: TeeWriter { a: file, b: stdout() }
        }
    }
);
```

This lets you:

- store different writer types in the same collection
- choose writers at runtime
- pass writers around behind pointers

But you pay a small cost:

- one pointer indirection
- one vtable lookup per call

For most I/O pipelines, static dispatch is better — and your example uses it perfectly.
