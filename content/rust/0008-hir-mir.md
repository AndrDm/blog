---
title: Understanding How Rust’s Borrow Checker Sees “Alive” and “Dead” References Using HIR and MIR
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

Rust’s borrow checker is famously strict, but also famously precise. When something fails to compile due to borrowing rules, the compiler is not guessing — it is analyzing your code through several internal representations, especially **HIR** (High‑level Intermediate Representation) and **MIR** (Mid‑level Intermediate Representation).

<!--more-->

## **The Example**

```rust
fn main() {
    let mut test_var = 100;
    dbg!(test_var);
    let test_var_ref = &mut test_var;
    // dbg!(test_var); // error
    dbg!(test_var_ref);
    dbg!(test_var);
}
```

This code fails if you uncomment the marked line:

```rust
fn main()
    let mut test_var = 100;
    dbg!(test_var);
    let test_var_ref = &mut test_var;
    dbg!(test_var); // error
    dbg!(test_var_ref);
    dbg!(test_var);
}
```

Result:

```
C:\Users\Andrey\Desktop\r-hir3\src>cargo run
   Compiling r-hir3 v0.1.0 (C:\Users\Andrey\Desktop\r-hir3)
error[E0503]: cannot use `test_var` because it was mutably borrowed
 --> src\main.rs:5:5
  |
4 |     let test_var_ref = &mut test_var;
  |                        ------------- `test_var` is borrowed here
5 |     dbg!(test_var); // error
  |     ^^^^^^^^^^^^^^ use of borrowed `test_var`
6 |     dbg!(test_var_ref);
  |          ------------ borrow later used here

For more information about this error, try `rustc --explain E0503`.
error: could not compile `r-hir3` (bin "r-hir3") due to 1 previous error
```

The question is: **why does the borrow checker still consider** `test_var` **borrowed at that point?**   To answer that, we need to look at HIR and MIR.

# **HIR: The High‑Level View**

HIR is a normalized, cleaned‑up version of your source code. It still resembles Rust closely, but with syntactic sugar removed and structure made explicit.

A simplified HIR for our example looks like this:

Code

```rust
fn main() {
    let mut test_var: i32 = 100;
    dbg!(test_var);
    let test_var_ref: &mut i32 = &mut test_var;
    dbg!(test_var_ref);
    dbg!(test_var);
}
```

HIR is not where borrow checking happens, but it defines:

- where references are created
- their types
- their scopes
- the order of expressions

Complete HIR output:

```
extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2024::*;
fn main() {
    let mut test_var = 100;
    // dbg!(test_var); // error
    match (test_var,) {
        args => {
            let (tmp,) = args;
            {
                ::std::io::_eprint({
                        super let args = (&(&&tmp as &dyn ::std::fmt::Debug),);
                        super let args = [format_argument::new_debug(args.0)];
                        unsafe {
                            format_arguments::new(b"\x1d[src\\main.rs:3:5] test_var = \xc1 \x00\x80`\x01\n\x00",
                                &args)
                        }
                    });
            };
            tmp
        }
    };
    let test_var_ref = &mut test_var;
    match (test_var_ref,) {
        args => {
            let (tmp,) = args;
            {
                ::std::io::_eprint({
                        super let args = (&(&&tmp as &dyn ::std::fmt::Debug),);
                        super let args = [format_argument::new_debug(args.0)];
                        unsafe {
                            format_arguments::new(b"![src\\main.rs:6:5] test_var_ref = \xc1 \x00\x80`\x01\n\x00",
                                &args)
                        }
                    });
            };
            tmp
        }
    };
    match (test_var,) {
        args => {
            let (tmp,) = args;
            {
                ::std::io::_eprint({
                        super let args = (&(&&tmp as &dyn ::std::fmt::Debug),);
                        super let args = [format_argument::new_debug(args.0)];
                        unsafe {
                            format_arguments::new(b"\x1d[src\\main.rs:7:5] test_var = \xc1 \x00\x80`\x01\n\x00",
                                &args)
                        }
                    });
            };
            tmp
        }
    };
}

```

This structure is then lowered into MIR.

# **MIR: The Borrow Checker’s Playground**

MIR is a much more explicit, SSA‑like (Static Single Assignment) representation. This is where the borrow checker performs its analysis.

Here is a **simplified MIR** for our function:

Code

```
bb0: {
    _1 = 100;                     // test_var
    _2 = dbg!(_1);                // dbg!(test_var)
    _3 = &mut _1;                 // test_var_ref = &mut test_var   <-- borrow starts
    _4 = dbg!(_3);                // dbg!(test_var_ref)             <-- borrow still active
    _5 = dbg!(_1);                // dbg!(test_var)                 <-- illegal use
    return;
}
```

Now let’s annotate it with borrow‑checker semantics.

# **Where the Borrow Checker Sees the Reference as “Alive”**

Code

```
_3 = &mut _1;                     // ← mutable borrow begins
_4 = dbg!(_3);                    // ← reference is still alive
_5 = dbg!(_1);                    // ← ERROR: _1 is still mutably borrowed here
```

The key rule:

> **A mutable borrow lives until its last use.**

The last use of `test_var_ref` is inside `dbg!(test_var_ref)`. Therefore, the borrow remains active until the end of that statement.

This is why the commented-out line fails:

rust

```
dbg!(test_var); // test_var is still mutably borrowed
```

Even though the reference was created earlier, its lifetime extends to its last use.

### Complete MIR output

```
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
// HINT: See also -Z dump-mir for MIR at specific points during compilation.
fn main() -> () {
    let mut _0: ();
    let mut _1: i32;
    let mut _2: (i32,);
    let mut _3: i32;
    let _5: ();
    let mut _6: std::fmt::Arguments<'_>;
    let mut _8: &&dyn std::fmt::Debug;
    let _9: &dyn std::fmt::Debug;
    let _10: &&i32;
    let _11: &i32;
    let mut _13: core::fmt::rt::Argument<'_>;
    let mut _14: &[u8; 38];
    let _15: &[core::fmt::rt::Argument<'_>; 1];
    let mut _17: (&mut i32,);
    let _19: ();
    let mut _20: std::fmt::Arguments<'_>;
    let mut _22: &&dyn std::fmt::Debug;
    let _23: &dyn std::fmt::Debug;
    let _24: &&&mut i32;
    let _25: &&mut i32;
    let mut _27: core::fmt::rt::Argument<'_>;
    let mut _28: &[u8; 42];
    let _29: &[core::fmt::rt::Argument<'_>; 1];
    let mut _30: (i32,);
    let mut _31: i32;
    let _33: ();
    let mut _34: std::fmt::Arguments<'_>;
    let mut _36: &&dyn std::fmt::Debug;
    let _37: &dyn std::fmt::Debug;
    let _38: &&i32;
    let _39: &i32;
    let mut _41: core::fmt::rt::Argument<'_>;
    let mut _42: &[u8; 38];
    let _43: &[core::fmt::rt::Argument<'_>; 1];
    let mut _44: &&dyn std::fmt::Debug;
    let mut _45: &&dyn std::fmt::Debug;
    let mut _46: &&dyn std::fmt::Debug;
    scope 1 {
        debug test_var => _1;
        let _16: &mut i32;
        scope 2 {
            debug args => _2;
            let _4: i32;
            scope 3 {
                debug tmp => _4;
                let _7: (&&dyn std::fmt::Debug,);
                scope 4 {
                    debug args => _7;
                    let _12: [core::fmt::rt::Argument<'_>; 1];
                    scope 5 {
                        debug args => _12;
                    }
                }
            }
        }
        scope 6 {
            debug test_var_ref => _16;
            scope 7 {
                debug args => _17;
                let _18: &mut i32;
                scope 8 {
                    debug tmp => _18;
                    let _21: (&&dyn std::fmt::Debug,);
                    scope 9 {
                        debug args => _21;
                        let _26: [core::fmt::rt::Argument<'_>; 1];
                        scope 10 {
                            debug args => _26;
                        }
                    }
                }
            }
            scope 11 {
                debug args => _30;
                let _32: i32;
                scope 12 {
                    debug tmp => _32;
                    let _35: (&&dyn std::fmt::Debug,);
                    scope 13 {
                        debug args => _35;
                        let _40: [core::fmt::rt::Argument<'_>; 1];
                        scope 14 {
                            debug args => _40;
                        }
                    }
                }
            }
        }
    }

    bb0: {
        _1 = const 100_i32;
        _3 = copy _1;
        _2 = (move _3,);
        _4 = copy (_2.0: i32);
        _11 = &_4;
        _10 = &_11;
        _9 = copy _10 as &dyn std::fmt::Debug (PointerCoercion(Unsize, AsCast));
        _8 = &_9;
        _7 = (move _8,);
        _44 = copy (_7.0: &&dyn std::fmt::Debug);
        _13 = core::fmt::rt::Argument::<'_>::new_debug::<&dyn Debug>(copy _44) -> [return: bb1, unwind continue];
    }

    bb1: {
        _12 = [move _13];
        _14 = const b"\x1d[src\\main.rs:3:5] test_var = \xc1 \x00\x80`\x01\n\x00";
        _15 = &_12;
        _6 = Arguments::<'_>::new::<38, 1>(move _14, copy _15) -> [return: bb2, unwind continue];
    }

    bb2: {
        _5 = std::io::_eprint(move _6) -> [return: bb3, unwind continue];
    }

    bb3: {
        _16 = &mut _1;
        _17 = (copy _16,);
        _18 = copy (_17.0: &mut i32);
        _25 = &_18;
        _24 = &_25;
        _23 = copy _24 as &dyn std::fmt::Debug (PointerCoercion(Unsize, AsCast));
        _22 = &_23;
        _21 = (move _22,);
        _45 = copy (_21.0: &&dyn std::fmt::Debug);
        _27 = core::fmt::rt::Argument::<'_>::new_debug::<&dyn Debug>(copy _45) -> [return: bb4, unwind continue];
    }

    bb4: {
        _26 = [move _27];
        _28 = const b"![src\\main.rs:6:5] test_var_ref = \xc1 \x00\x80`\x01\n\x00";
        _29 = &_26;
        _20 = Arguments::<'_>::new::<42, 1>(move _28, copy _29) -> [return: bb5, unwind continue];
    }

    bb5: {
        _19 = std::io::_eprint(move _20) -> [return: bb6, unwind continue];
    }

    bb6: {
        _31 = copy _1;
        _30 = (move _31,);
        _32 = copy (_30.0: i32);
        _39 = &_32;
        _38 = &_39;
        _37 = copy _38 as &dyn std::fmt::Debug (PointerCoercion(Unsize, AsCast));
        _36 = &_37;
        _35 = (move _36,);
        _46 = copy (_35.0: &&dyn std::fmt::Debug);
        _41 = core::fmt::rt::Argument::<'_>::new_debug::<&dyn Debug>(copy _46) -> [return: bb7, unwind continue];
    }

    bb7: {
        _40 = [move _41];
        _42 = const b"\x1d[src\\main.rs:7:5] test_var = \xc1 \x00\x80`\x01\n\x00";
        _43 = &_40;
        _34 = Arguments::<'_>::new::<38, 1>(move _42, copy _43) -> [return: bb8, unwind continue];
    }

    bb8: {
        _33 = std::io::_eprint(move _34) -> [return: bb9, unwind continue];
    }

    bb9: {
        return;
    }
}

alloc3 (size: 38, align: 1) {
    0x00 │ 1d 5b 73 72 63 5c 6d 61 69 6e 2e 72 73 3a 37 3a │ .[src\main.rs:7:
    0x10 │ 35 5d 20 74 65 73 74 5f 76 61 72 20 3d 20 c1 20 │ 5] test_var = . 
    0x20 │ 00 80 60 01 0a 00                               │ ..`...
}

alloc2 (size: 42, align: 1) {
    0x00 │ 21 5b 73 72 63 5c 6d 61 69 6e 2e 72 73 3a 36 3a │ ![src\main.rs:6:
    0x10 │ 35 5d 20 74 65 73 74 5f 76 61 72 5f 72 65 66 20 │ 5] test_var_ref 
    0x20 │ 3d 20 c1 20 00 80 60 01 0a 00                   │ = . ..`...
}

alloc1 (size: 38, align: 1) {
    0x00 │ 1d 5b 73 72 63 5c 6d 61 69 6e 2e 72 73 3a 33 3a │ .[src\main.rs:3:
    0x10 │ 35 5d 20 74 65 73 74 5f 76 61 72 20 3d 20 c1 20 │ 5] test_var = . 
    0x20 │ 00 80 60 01 0a 00                               │ ..`...
}

```



# **Why the Borrow Lives Longer Than You Expect**

Many developers assume that a borrow ends at the end of the line where the reference is created. But Rust’s borrow checker is more precise:

- A borrow ends when the **reference itself** is no longer used.
- Not when the reference is created.
- Not when the variable goes out of scope.
- Not when the block ends (unless that block contains the last use).

In our example:

```rust
let test_var_ref = &mut test_var;
dbg!(test_var_ref);   // last use
dbg!(test_var);       // still illegal here
```

The borrow ends **after** the `dbg!(test_var_ref)` call completes.

# **How to Make the Borrow End Earlier**

If you want the borrow to end sooner, you must limit the reference’s scope:

```rust
{
    let test_var_ref = &mut test_var;
    dbg!(test_var_ref);
} // ← borrow ends here

dbg!(test_var); // now allowed
```

This creates a separate MIR region, and the borrow checker can clearly see that the reference is dead before the final `dbg!`.

# **Conclusion**

Rust’s borrow checker is strict, but it is also predictable once you understand how it interprets your code through HIR and MIR. The key insight is that **a borrow lives until the last use of the reference**, not until the line where it was created.

By examining MIR, you can see exactly where Rust considers a reference alive or dead — and why certain patterns compile while others don’t.

### Built with

```
cargo +nightly rustc -- -Zunpretty=hir > hir.log
cargo +nightly rustc -- -Zunpretty=mir > mir.log
```

