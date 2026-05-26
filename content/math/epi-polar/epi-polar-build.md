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

Visual Studio Professional 2026 [v.18.6.1](https://learn.microsoft.com/en-us/visualstudio/releases/2026/release-notes#18.6.1)

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

setlocal
cd C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSetGui
if %errorlevel% neq 0 goto :cmEnd
C:
if %errorlevel% neq 0 goto :cmEnd
"C:\Program Files\CMake\bin\cmake.exe" -E cmake_autogen C:/Users/Andrey/Desktop/epi-polar/LibGetSet-dev/build/GetSetGui/CMakeFiles/GetSetGui_autogen.dir/AutogenInfo.json Release
if %errorlevel% neq 0 goto :cmEnd
:cmEnd
endlocal & call :cmErrorLevel %errorlevel% & goto :cmDone
:cmErrorLevel
exit /b %1
:cmDone
if %errorlevel% neq 0 goto :VCEnd

---

setlocal
cd C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSetGui
if %errorlevel% neq 0 goto :cmEnd
C:
if %errorlevel% neq 0 goto :cmEnd
"C:\Program Files\CMake\bin\cmake.exe" -E cmake_autogen C:/Users/Andrey/Desktop/epi-polar/LibGetSet-dev/build/GetSetGui/CMakeFiles/GetSetGui_autogen.dir/AutogenInfo.json Debug
if %errorlevel% neq 0 goto :cmEnd
:cmEnd
endlocal & call :cmErrorLevel %errorlevel% & goto :cmDone
:cmErrorLevel
exit /b %1
:cmDone
if %errorlevel% neq 0 goto :VCEnd

Severity	Code	Description	Project	File	Line	Suppression State	Details
Error	LNK1104	cannot open file 'C:/Users/Andrey/Desktop/epi-polar/LibGetSet-dev/build/GetSetGui/GetSetGui.dir/Debug/exports.def'	GetSetGui	C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSetGui\LINK	1		

---

C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSet\Debug\GetSetd.lib;

C:\Qt\6.10.2\lib\Qt6Widgetsd.lib;

C:\Qt\6.10.2\lib\Qt6Guid.lib;

C:\Qt\6.10.2\lib\Qt6Cored.lib;



mpr.lib;userenv.lib;d3d11.lib;dxgi.lib;dxguid.lib;d3d12.lib;kernel32.lib;user32.lib;gdi32.lib;winspool.lib;shell32.lib;ole32.lib;oleaut32.lib;uuid.lib;comdlg32.lib;advapi32.lib

C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSet\Debug\GetSetd.lib;

C:\Qt\6.10.2\lib\Qt6Widgetsd.lib;

C:\Qt\6.10.2\lib\Qt6Guid.lib;

C:\Qt\6.10.2\lib\Qt6Cored.lib;

mpr.lib;userenv.lib;d3d11.lib;dxgi.lib;dxguid.lib;d3d12.lib;kernel32.lib;user32.lib;gdi32.lib;winspool.lib;shell32.lib;ole32.lib;oleaut32.lib;uuid.lib;comdlg32.lib;advapi32.lib

C:\Users\Andrey\Desktop\epi-polar\LibGetSet-dev\build\GetSet\Release\GetSet.lib;

C:\Qt\6.10.2\lib\Qt6Widgets.lib;

C:\Qt\6.10.2\lib\Qt6Gui.lib;

C:\Qt\6.10.2\lib\Qt6Core.lib;

mpr.lib;userenv.lib;d3d11.lib;dxgi.lib;dxguid.lib;d3d12.lib;kernel32.lib;user32.lib;gdi32.lib;winspool.lib;shell32.lib;ole32.lib;oleaut32.lib;uuid.lib;comdlg32.lib;advapi32.lib

---

Selecting Windows SDK version 10.0.26100.0 to target Windows 10.0.26200.

The C compiler identification is MSVC 19.51.36244.0

The CXX compiler identification is MSVC 19.51.36244.0

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

NLopt version 2.10.1

Looking for getopt.h

Looking for getopt.h - not found

Looking for unistd.h

Looking for unistd.h - not found

Looking for stdint.h

Looking for stdint.h - found

Looking for time.h

Looking for time.h - found

Looking for sys/time.h

Looking for sys/time.h - not found

Looking for getpid

Looking for getpid - found

Looking for syscall

Looking for syscall - not found

Looking for isinf

Looking for isinf - not found

Looking for isnan

Looking for isnan - not found

Looking for gettimeofday

Looking for gettimeofday - not found

Looking for qsort_r

Looking for qsort_r - not found

Looking for time

Looking for time - found

Looking for copysign

Looking for copysign - found

Looking for getopt

Looking for getopt - not found

Looking for sys/types.h

Looking for sys/types.h - found

Looking for stddef.h

Looking for stddef.h - found

Check size of uint32_t

Check size of uint32_t - done

Check size of unsigned int

Check size of unsigned int - done

Check size of unsigned long

Check size of unsigned long - done

Looking for fpclassify

Looking for fpclassify - FALSE

Performing Test HAVE_THREAD_LOCAL_STORAGE

Performing Test HAVE_THREAD_LOCAL_STORAGE - Failed

Performing Test HAVE_THREAD_LOCAL_STORAGE

Performing Test HAVE_THREAD_LOCAL_STORAGE - Success

Could NOT find Python (missing: Python_NumPy_INCLUDE_DIRS NumPy) (found suitable version "3.14.2", minimum required is "3.6")

Could NOT find Guile (missing: GUILE_EXECUTABLE GUILE_ROOT_DIR GUILE_INCLUDE_DIRS GUILE_LIBRARIES) 

Could NOT find JNI (missing: JAVA_INCLUDE_PATH JAVA_INCLUDE_PATH2 JVM) 

Could NOT find Java (missing: Java_JAVA_EXECUTABLE Java_JAR_EXECUTABLE Java_JAVAC_EXECUTABLE Java_JAVAH_EXECUTABLE Java_JAVADOC_EXECUTABLE) (Required is at least version "1.5")

Could NOT find SWIG (missing: SWIG_EXECUTABLE SWIG_DIR) (Required is at least version "3")

Could NOT find Octave (missing: OCTAVE_EXECUTABLE OCTAVE_ROOT_DIR OCTAVE_INCLUDE_DIRS OCTAVE_LIBRARIES) 

Configuring done (40.6s)

---

Selecting Windows SDK version 10.0.26100.0 to target Windows 10.0.26200.

NLopt version 2.10.1

Could NOT find Python (missing: Python_NumPy_INCLUDE_DIRS NumPy) (found suitable version "3.14.2", minimum required is "3.6")

Could NOT find Guile (missing: GUILE_EXECUTABLE GUILE_ROOT_DIR GUILE_INCLUDE_DIRS GUILE_LIBRARIES) 

Could NOT find JNI (missing: JAVA_INCLUDE_PATH JAVA_INCLUDE_PATH2 JVM) 

Could NOT find Java (missing: Java_JAVA_EXECUTABLE Java_JAR_EXECUTABLE Java_JAVAC_EXECUTABLE Java_JAVAH_EXECUTABLE Java_JAVADOC_EXECUTABLE) (Required is at least version "1.5")

Could NOT find SWIG (missing: SWIG_EXECUTABLE SWIG_DIR) (Required is at least version "3")

Could NOT find Octave (missing: OCTAVE_EXECUTABLE OCTAVE_ROOT_DIR OCTAVE_INCLUDE_DIRS OCTAVE_LIBRARIES) 

Configuring done (0.9s)

