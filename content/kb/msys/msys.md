---
title: How to indtsll gcc and clang on Windows using MSYS
date: 2025-04-30
authorbox: false
sidebar: false
description: MSys installation guide
categories:
  - Knowledge Base
archives: 2025-04
tags:
  - MSYS
draft: false
---
Installation of gcc and clang is not very complicated, but there are some issues in corporate environment which I would like to note.
<!--more-->
A very first step of course, download in install MSYS2

**MSYS2** is a collection of tools and libraries providing you with an easy-to-use environment for building, installing and running native Windows software.

### Download MSYS2

Home page: https://www.msys2.org/

Download link: [msys2-x86_64-20250221.exe](https://github.com/msys2/msys2-installer/releases/download/2025-02-21/msys2-x86_64-20250221.exe)

Installing MSYS2 requires 64 bit Windows 10 or newer.

### Install GCC and CLANG

There are only few commands needs to be executed in the terminal

```
pacman -S mingw-w64-ucrt-x86_64-gcc
pacman -S mingw-w64-ucrt-x86_64-clang
```

### Pacman update

```
pacman -Sy
pacman -Syu
```

### Add to PATH

This folder **C:\msys64\ucrt64\bin** needs to be added to Path Environment Variable.

### Check

From command prompt execute to get current versions (as per 30-APR-2025):

#### GCC:

```
gcc -v
Using built-in specs.
COLLECT_GCC=C:\msys64\ucrt64\bin\gcc.exe
...
Thread model: posix
Supported LTO compression algorithms: zlib zstd
gcc version 15.1.0 (Rev1, Built by MSYS2 project)
```

#### CLANG:

```
clang -v
clang version 20.1.3
Target: x86_64-w64-windows-gnu
Thread model: posix
InstalledDir: C:/msys64/ucrt64/bin
```

### Troubleshooting

If you have troubles with SSL, then this check can be disabled in C:\msys64\etc\pacman.conf:

```
# /etc/pacman.conf
#
# GENERAL OPTIONS
#
[options]
HoldPkg     = pacman
# SSL AND SIGNAURE OFF
XferCommand = /usr/bin/curl -k -L -o %o %u
LocalFileSigLevel = Never
RemoteFileSigLevel = Never
SigLevel = Never
...
Architecture = auto
Color
CheckSpace
ParallelDownloads = 5
```

Local database located in **C:\msys64\var\lib\pacman\sync**. Delete, if gets damaged by firewall.

Mirrorlists are located here: **C:\msys64\etc\pacman.d\mirrorlist.***

You may need to keep here the servers only where you have access to:

```
# See https://www.msys2.org/dev/mirrors

## Primary
Server = https://repo.msys2.org/mingw/clang64/
```

