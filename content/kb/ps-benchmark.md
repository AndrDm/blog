---
title: Very quick "Benchmark" with Power Shell
date: 2024-12-02
authorbox: false
sidebar: false
description: Very quick "Benchmark" with Power Shell
categories:
  - Knowledge Base
archives:
  - 2024-12
tags:
  - Benchmark
draft: false
---
Time to time I need "quick and dirty" benchmark to check performance. Sometimes using CPU Bench from CPU-Z or AIDA, but there is quicker way with Power Shell...
<!--more-->
The script is the following:

```powershell
Measure-Command {
        $array = New-Object System.Collections.ArrayList
        for ($i = 0; $i -lt 5000000; $i++) {
          [void]$array.Add($i)
        }
        $array.Sort()
      }
```

And the output is

```
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 8
Milliseconds      : 576
Ticks             : 85764768
TotalDays         : 9,92647777777778E-05
TotalHours        : 0,00238235466666667
TotalMinutes      : 0,14294128
TotalSeconds      : 8,5764768
TotalMilliseconds : 8576,4768
```

Originally from [Vmware 17 Pro very slow on Windows 11 22H2](https://community.broadcom.com/vmware-cloud-foundation/discussion/vmware-17-pro-very-slow-on-windows-11-22h2).
