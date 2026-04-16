---
title: Rust - Macro vs Function
date: 2026-04-12
description: Macro vs Function
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
A practical look at when to use macros and when to stick with functions in Rust, using a real FFI example (`void_ptr`) to show why macros don’t magically make your code faster — and why small helper functions often compile to identical machine code. 
<!--more-->

Let’s say I have an FFI call like this:

```rust
unsafe extern "C" {
	pub fn CNVGetScalarDataValue(
		data: CNVData,
		type_: CNVDataType,
		value: *mut ::std::os::raw::c_void,
	) -> ::std::os::raw::c_int;
}
```

This comes from CVI Network Variables:

```c
int CVIFUNC CNVGetScalarDataValue(CNVData data, CNVDataType type, void * value);
```

The function supports different scalar types, so in Rust it might be called like this:

```
	//...
	let mut value: c_int = 0;
	let result = CNVGetScalarDataValue(
		data,
		CNVDataType_CNVInt32,
		&mut value as *mut c_int as *mut c_void,
	);
	println!("{}", value);
```

It works, but the expression `&mut value as *mut c_int as *mut c_void` isn’t exactly pleasant to look at. Making the cast more idiomatic

If `value` is already a `c_int`, you can shorten the casts:

```rust
&mut value as *mut _ as *mut c_void
```

or even

```rust
value.as_mut() as *mut _ as *mut c_void
```

📌 Why this is the minimum

- `CNVGetScalarDataValue` expects a `*mut c_void`
- Rust references (`&mut T`) must be cast to raw pointers (`*mut T`)
- Then cast again to `*mut c_void`

So you always need **two casts**, but you can shorten the types.

⭐ Even cleaner: helper function

If you call this often, hide the ugliness:

```rust
fn void_ptr<T>(v: &mut T) -> *mut c_void {
    v as *mut T as *mut c_void
}

let result = CNVGetScalarDataValue(
    data,
    CNVDataType_CNVInt32,
    void_ptr(&mut value),
);
```

This is the most readable approach, but our goal is simply to avoid writing the casts manually, this is the closest macro equivalent:

⭐ Slightly nicer: macro that takes value and borrows it
If you want the macro to always take a mutable reference:

```rust
macro_rules! void_ptr {
    ($v:expr) => {
        &mut $v as *mut _ as *mut c_void
    };
}
```

Then usage:

```rust
let result = CNVGetScalarDataValue(
    data,
    CNVDataType_CNVInt32,
    void_ptr!(value),
);
```

Are macros faster than functions? Short answer: **no**. Rust aggressively inlines small generic functions, so the helper function and the macro produce identical machine code.

Here’s the relevant assembly from both versions:

```assembly
.text:0000000140001266                 mov     [rbp+60h+var_4C], 0
.text:000000014000126D                 mov     rcx, qword ptr [rbp+60h+var_78]
.text:0000000140001271                 mov     edx, 3
.text:0000000140001276                 mov     r8, rdi
.text:0000000140001279                 call    CNVGetScalarDataValue
.text:000000014000127E                 mov     r12d, eax
.text:0000000140001281                 mov     [rbp+60h+var_50], 0
.text:0000000140001288                 mov     rcx, qword ptr [rbp+60h+var_78]
.text:000000014000128C                 mov     edx, 3
.text:0000000140001291                 mov     r8, rbx
.text:0000000140001294                 call    CNVGetScalarDataValue
```

The pointer argument ends up in `r8` in both cases, and there is **no difference at all** between the macro and the function.

## Conclusion

Macros are great for syntactic convenience, but they don’t make your code faster. For FFI pointer casts, a small helper function is usually the cleanest and safest option — and thanks to Rust’s optimizer, it compiles down to the same machine code as a macro.

## Links

[Rust Macros vs Functions: What Java and Python Developers Should Know](https://noos.blog/posts/rust-macros-vs-functions/).
