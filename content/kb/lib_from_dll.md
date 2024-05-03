---
title: How to create *.lib file from *.dll
date: 2024-05-03
authorbox: false
sidebar: false
description: How to create *.lib file from *.dll
categories:
  - Programming
archives:
  - 2024-05
tags:
  - DLL
draft: false
---
Two command lines which I often forgot and googling again and again. If you have dynamic link library, but haven't according \*.lib file, then use

```
dumpbin /EXPORTS yourfile.dll > yourfile.exports
lib /def:yourfile.def /out:yourfile.lib
```

<!--more-->
For example, you would like to call some functions from NIVisSvc.dll from C Code and therefore need to link to this library.

SO: [How to make a .lib file when have a .dll file and a header file](https://stackoverflow.com/questions/9360280/how-to-make-a-lib-file-when-have-a-dll-file-and-a-header-file)

1. Run Visual Studio Developer Command Prompt:
```
C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars32.bat
```
or for x64
```
C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat
```
2. `dumpbin /EXPORTS yourfile.dll > yourfile.exports`

3. Paste the names of the needed functions from `yourfile.exports` into a new `yourfile.def` file. Add a line with the word `EXPORTS` at the top of this file.

4. Run the following commands
```
lib /def:yourfile.def /out:yourfile.lib
```
or for x64 builds

```
lib /def:yourfile.def /machine:x64 /out:yourfile64.lib
```
or instead of calling this from VS Dev Command Prompt you can call these tools from the folder where where `lib.exe` and other compile tools reside:

```
C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64\
```
You should get two files generated: `yourfile.lib` and `yourfile.exp`

### Practical Exercise

Create NIVisSvc.lib from NIVisSvc.dll:

x64:

```
copy C:\Windows\System32\NIVisSvc.dll NIVisSvc.dll
dumpbin /EXPORTS NIVisSvc.dll > NIVisSvc.exports
lib /def:NIVisSvc.def /machine:x64 /out:NIVisSvc.lib
```

x86:

```
copy C:\Windows\SysWOW64\NIVisSvc.dll NIVisSvc.dll
dumpbin /EXPORTS NIVisSvc.dll > NIVisSvc.exports
lib /def:NIVisSvc.def /machine:x86 /out:NIVisSvc.lib
```

\*.exports file

```
Microsoft (R) COFF/PE Dumper Version 14.39.33523.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file NIVisSvc.dll

File Type: DLL

  Section contains the following exports for nivissvc.dll

    00000000 characteristics
    65A21E95 time date stamp Sat Jan 13 06:24:37 2024
        0.00 version
           1 ordinal base
         693 number of functions
         367 number of names

    ordinal hint RVA      name

         23    0 006F71C4 ?CVI_ProcessError_OBSOLETE_1@@3P6A?AW4GRLIBError_enum@@W41@IZZA
         32    1 006F71D8 ?LV_GetFunctionPtr@@3P6A?AW4GRLIBError_enum@@PBDPAPAX@ZA
         51    2 00535EEE ALic
         52    3 00370210 AddAnnulusContour
         53    4 003702D0 AddClosedContour
         54    5 00370320 AddContourToROIContourList
...
```

modified to \*.def:

```
EXPORTS
ALic
AddAnnulusContour
AddClosedContour
AddContourToROIContourList
AddEntry
...
```

For modification I would to recommend any editor, which allow to delete column of the text

In VSCode hold <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>Alt</kbd> and select text with keyboard only using arrows keys.

In Far Manager use <kbd>Shift</kbd>+<kbd>Alt</kbd>

