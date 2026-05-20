---
title: Build and explore Epipolar Consistency of X-Ray Images
date: 2026-05-20
description: Epipolar Consistency
programming_languages:
  - Rust
  - C++
categories:
  - Programming
archives:
  - 2026-05
tags:
  - Math
draft: false
---

Just notes  about Epipolar Consistency of X-Ray Images

<!--more-->

Base original Article — [Epipolar Consistency](https://www5.cs.fau.de/research/software/epipolar-consistency/).

Base GitHub Repository — [Epipolar Consistency of X-Ray Images](https://github.com/aaichert/EpipolarConsistency).

## Software 

Visual Studio Professional 2026 [v.18.6.0](https://learn.microsoft.com/en-us/visualstudio/releases/2026/release-notes#18.6.0)

[CMake](https://cmake.org/download/) [4.3.2](https://github.com/Kitware/CMake/releases/download/v4.3.2/cmake-4.3.2-windows-x86_64.msi)

## Build

First atttempt:

<details>
  <summary>NLopt Required</summary>

```

Selecting Windows SDK version 10.0.26100.0 to target Windows 10.0.26200.
The C compiler identification is MSVC 19.51.36243.0
The CXX compiler identification is MSVC 19.51.36243.0
Detecting C compiler ABI info
Detecting C compiler ABI info - done
Check for working C compiler: C:/Program Files/Microsoft Visual Studio/18/Professional/VC/Tools/MSVC/14.51.36231/bin/Hostx64/x64/cl.exe - skipped
Detecting C compile features
Detecting C compile features - done
Detecting CXX compiler ABI info
Detecting CXX compiler ABI info - done
Check for working CXX compiler: C:/Program Files/Microsoft Visual Studio/18/Professional/VC/Tools/MSVC/14.51.36231/bin/Hostx64/x64/cl.exe - skipped
Detecting CXX compile features
Detecting CXX compile features - done
Added parallel build arguments to CMAKE_CXX_FLAGS: /DWIN32 /D_WINDOWS /EHsc /MP
Found OpenMP_C: -openmp (found version "2.0")
Found OpenMP_CXX: -openmp (found version "2.0")
Found OpenMP: TRUE (found version "2.0")
CMake Error at CMakeLists.txt:79 (find_package):
  By not providing "FindNLopt.cmake" in CMAKE_MODULE_PATH this project has
  asked CMake to find a package configuration file provided by "NLopt", but
  CMake did not find one.

  Could not find a package configuration file provided by "NLopt" with any of
  the following names:

    NLopt.cps
    nlopt.cps
    NLoptConfig.cmake
    nlopt-config.cmake

  Add the installation prefix of "NLopt" to CMAKE_PREFIX_PATH or set
  "NLopt_DIR" to a directory containing one of the above files.  If "NLopt"
  provides a separate development package or SDK, be sure it has been
  installed.

Configuring incomplete, errors occurred!

```

</details>

