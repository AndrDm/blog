---
title: Actual List of Rust Targets
date: 2026-07-04
description: targets
programming_languages:
  - Rust
categories:
  - Programming
archives:
  - 2026-07
tags:
  - INFO
draft: false
---

Rust’s cross-compilation story is one of its biggest strengths. Whether you’re building for embedded devices, mobile platforms, game consoles, or cloud environments, Rust provides a wide range of targets that define how your code is compiled and what platform it runs on.

This post collects the actual list of targets available on my system, along with quick instructions on how to list, install, and remove toolchains using rustup.

<!--more-->

A Rust target describes the architecture, operating system, ABI, and environment your compiled binary is meant for. If you’re working with cross‑platform development, embedded systems, or experimenting with new architectures, knowing your available targets is essential.

Below is the full output of:

```Console
rustup target list
```

Output 

```
aarch64-apple-darwin
aarch64-apple-ios
aarch64-apple-ios-macabi
aarch64-apple-ios-sim
aarch64-apple-tvos
aarch64-apple-tvos-sim
aarch64-apple-visionos
aarch64-apple-visionos-sim
aarch64-apple-watchos
aarch64-apple-watchos-sim
aarch64-linux-android
aarch64-pc-windows-gnullvm
aarch64-pc-windows-msvc
aarch64-unknown-fuchsia
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
aarch64-unknown-linux-ohos
aarch64-unknown-none
aarch64-unknown-none-softfloat
aarch64-unknown-uefi (installed)
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
arm64ec-pc-windows-msvc
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv7-linux-androideabi
armv7-unknown-linux-gnueabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
armv7-unknown-linux-ohos
armv7a-none-eabi
armv7a-none-eabihf
armv7r-none-eabi
armv7r-none-eabihf
armv8r-none-eabihf
i586-unknown-linux-gnu
i586-unknown-linux-musl
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-gnullvm
i686-pc-windows-msvc (installed)
i686-unknown-freebsd
i686-unknown-linux-gnu
i686-unknown-linux-musl
i686-unknown-uefi (installed)
loongarch64-unknown-linux-gnu
loongarch64-unknown-linux-musl
loongarch64-unknown-none
loongarch64-unknown-none-softfloat
nvptx64-nvidia-cuda
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64-unknown-linux-musl
powerpc64le-unknown-linux-gnu
powerpc64le-unknown-linux-musl
riscv32i-unknown-none-elf
riscv32im-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imafc-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64a23-unknown-linux-gnu
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-linux-musl
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
s390x-unknown-linux-gnu
sparc64-unknown-linux-gnu
sparcv9-sun-solaris
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
wasm32-unknown-emscripten
wasm32-unknown-unknown (installed)
wasm32-wasip1
wasm32-wasip1-threads
wasm32-wasip2
wasm32v1-none
x86_64-apple-darwin
x86_64-apple-ios
x86_64-apple-ios-macabi
x86_64-fortanix-unknown-sgx
x86_64-linux-android
x86_64-pc-solaris
x86_64-pc-windows-gnu
x86_64-pc-windows-gnullvm
x86_64-pc-windows-msvc (installed)
x86_64-unknown-freebsd
x86_64-unknown-fuchsia
x86_64-unknown-illumos
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnuasan
x86_64-unknown-linux-gnumsan
x86_64-unknown-linux-gnutsan
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl
x86_64-unknown-linux-ohos
x86_64-unknown-netbsd
x86_64-unknown-none
x86_64-unknown-redox
x86_64-unknown-uefi (installed)
```

How to install, for example, nightly build

```
rustup toolchain install nightly-x86_64-pc-windows-msvc
```

Output:

```
C:\Users\Andrey>rustup toolchain install nightly-x86_64-pc-windows-msvc
info: syncing channel updates for nightly-x86_64-pc-windows-msvc
info: latest update on 2026-07-04 for version 1.98.0-nightly (c397dae80 2026-07-02)
info: downloading 6 components
        cargo installed                        9.76 MiB
       clippy installed                        3.77 MiB
    rust-docs installed                       23.02 MiB
     rust-std installed                       21.93 MiB
        rustc installed                       68.04 MiB
      rustfmt installed                        2.46 MiB
  nightly-x86_64-pc-windows-msvc installed - rustc 1.98.0-nightly (c397dae80 2026-07-02)

info: checking for self-update (current version: 1.29.0)
```

How to uninstall

```Bash
rustup toolchain uninstall nightly-x86_64-pc-windows-msvc
```

Documentation:

```
C:\Users\Andrey\Desktop\blog\content\rust>rustup toolchain
Install, uninstall, or list toolchains

Usage: rustup[EXE] toolchain <COMMAND>

Commands:
  list       List installed toolchains
  install    Install or update the given toolchains, or by default the active toolchain
  uninstall  Uninstall the given toolchains
  link       Create a custom toolchain by symlinking to a directory
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

Discussion:
  Many `rustup` commands deal with *toolchains*, a single
  installation of the Rust compiler. `rustup` supports multiple
  types of toolchains. The most basic track the official release
  channels: 'stable', 'beta' and 'nightly'; but `rustup` can also                                           07:48
  install specific toolchains from the official archives, toolchains for
  alternate host platforms, and from local builds ('custom toolchains').

  Standard release channel toolchain names have the following form:

    <channel>[-<date>][-<host>]

    <channel>       = stable|beta|nightly|<versioned>[-<prerelease>]
    <versioned>     = <major.minor>|<major.minor.patch>
    <prerelease>    = beta[.<number>]
    <date>          = YYYY-MM-DD
    <host>          = <target-triple>

  'channel' is a named release channel, a major and minor version
  number such as `1.42`, or a fully specified version number, such
  as `1.42.0`. Channel names can be optionally appended with an
  archive date, as in `nightly-2014-12-18`, in which case the
  toolchain is downloaded from the archive for that date.

  The host may be specified as a target tuple. This is most useful
  for installing a 32-bit compiler on a 64-bit platform, or for
  installing the [MSVC-based toolchain] on Windows. For example:

    $ rustup toolchain install stable-x86_64-pc-windows-msvc

  For convenience, omitted elements of the target tuple will be
  inferred, so the above could be written:

    $ rustup toolchain install stable-msvc

  The `rustup default` command may be used to both install and set
  the desired toolchain as default in a single command:

    $ rustup default stable-msvc

  rustup can also manage symlinked local toolchain builds, which are
  often used for developing Rust itself. For more information see
  `rustup toolchain help link`.
```

