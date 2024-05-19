---
title: Large File Experiment
date: 2024-05-19
authorbox: false
sidebar: false
description: Experiment with Large File - we will write few GBs file, then read it with multiple threads
categories:
  - Programming
programming_languages:
  - LabVIEW
archives:
  - 2024-05
tags:
  - Benchmark
draft: false
---
In this "weekend" experiment I'll write large file (16 GB in my case), then read it back with different buffer size and threads and check the performance.
<!--more-->
The result is more or less predictable — each drive have a given throughput limit, and we can't read faster than drive and interface will allow, but there are some interesting points which I would like to share.

To avoid read from cache I will create file, which is equal to RAM's Size. In my case test laptop equipped with 16GB RAM, so I'll write 16 GB File.

## Write Experiment

Writing speed measurement is slightly out of scope, but anyway I will check how fast this will take.

I will write test file with 1MB blocks:

![](assets/write_snippet.png)

Now the results. for 16 GB it takes 86,7 seconds, means 189 MB/s average speed:

![image-20240519065650323](assets/image-20240519065650323.png)

Interesting point that the first GB was written very fast — around 2GB/s. Of course my SSD (which Samsung 750 EVO) can't hold this speed. There are three factors — first, I have Samsung Magician software with enabled RAPID mode, then, second, obviously we have OS file write cache, and third, SSD's cache. All three together resulted that the fist GB written to RAM, then dropped to SSD. This is what happened after first GB:

![image-20240519070725791](assets/image-20240519070725791.png)

Theoretically I can turn off Rapid Mode, as well as write caching, but this experiment is slightly out of scope (expected some performance drop anyway).

## Read Experiment

Now we will read large file back with standard LabVIEW's File I/O functions. There are two parameters which can be changed — block size to be read in single operation and amount of threads. My test laptop have 4 cores, so I'll check from single to 4 threads and iterate over different block sizes starting from from 512 bytes:

![](assets/write_snippet-1716103299429-1.png)

And the results (measured in MB/s), Samsung 870 EVO on i7-7700:

|        | 1 Thread | 2 Threads | 3 Threads | 4 Threads |
| ------ | -------- | --------- | --------- | --------- |
| 512 B  | 94,68    | 125,01    | 70,04     | 89,04     |
| 2 KB   | 356,69   | 105,70    | 153,52    | 185,32    |
| 8 KB   | 400,14   | 323,52    | 379,25    | 411,00    |
| 32 KB  | 394,29   | 445,07    | 311,05    | 410,40    |
| 128 KB | 461,78   | 403,30    | 439,52    | 523,10    |
| 512 KB | 484,78   | 499,46    | 531,30    | 531,77    |
| 2 MB   | 462,56   | 530,17    | 531,63    | 531,53    |
| 8 MB   | 475,75   | 529,21    | 530,33    | 530,14    |

This measurement can be different on different SSDs and PC Configurations, of course.

But in general this is expected results, the same can be obtained with traditional benchmark, for example:

![image-20240519092710162](assets/image-20240519092710162.png)

With larger amount of the threads we can reach "saturation" limit earlier with smaller buffer. And for better performance the buffer should be large enough - 256 KB - 1 MB.
