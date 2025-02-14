---
title: How to run DeepSeek-R1 on Xeon 6132 with 768 GB RAM
date: 2025-02-14
authorbox: false
sidebar: false
description: Very quick "Benchmark"
categories:
  - Knowledge Base
  - AI
archives: 2025-02
tags:
  - Benchmark
  - DeepSeek
draft: false
---
To get DeepSeek running on your PC you will need enough RAM, and it is quite easy with llama.cpp. Below step by step guide and results.
<!--more-->
PC HP z8 G4 Workstation.
CPU:

![https://habrastorage.org/r/w1560/webt/d_/yh/_p/d_yh_p0en_whozfuj99jqozqxsy.png](https://habrastorage.org/r/w1560/webt/d_/yh/_p/d_yh_p0en_whozfuj99jqozqxsy.png)

Memory - 12 x 64 GB = 768 GB:

![](https://habrastorage.org/r/w1560/webt/on/5g/v0/on5gv0rl5mzjikwejwxcuempii0.png)

Memory Benchmark (take a note, this is 12-channels memory with around 180 GB/s read - this is important for performance):

![](https://habrastorage.org/r/w1560/webt/r2/pr/ru/r2prrupxqhuzmzpu60l2rporwd0.png)

Video Adapters (will be unused):

![https://habrastorage.org/r/w1560/webt/pn/yp/nx/pnypnxaxpnehsvrmwgw8nxb-wau.png](https://habrastorage.org/r/w1560/webt/pn/yp/nx/pnypnxaxpnehsvrmwgw8nxb-wau.png)

SSD Drive:

![](https://habrastorage.org/r/w1560/webt/gu/xu/sy/guxusypp7qprdup0zmk7_jycr7o.png)

### Download DeepSeek

I will download full [DeepSeek R1 Q8_0](https://huggingface.co/unsloth/DeepSeek-R1-GGUF/tree/main/DeepSeek-R1-Q8_0) from the HuggingFace [DeepSeek-R1-GGUF](https://huggingface.co/unsloth/DeepSeek-R1-GGUF/tree/main) Repository.

### Download llama.cpp

Windows builds available in the [Releases](https://github.com/ggerganov/llama.cpp/releases). I downloaded AVX-512 version.

### Run it

```
llama-cli.exe -m DeepSeek-R1.Q8_0-00001-of-00015.gguf -p "Hi!" -n 4096
```

That is. Also tried ollama and LM Studio, but got some troubles and issues. 

How it looks in real time:

![](https://habrastorage.org/webt/fw/dy/na/fwdyna4ul4ucut-zug3yhonhhwc.gif)

Let try to ask C Source code for Runge-Kutta Method:

![](https://habrastorage.org/webt/bv/w8/db/bvw8db0gki2rnbcr1k77z3lih-u.gif)

It is not very fast, around 1.2 tokens per second, but works.

It recommended to turn Hyper threading Off to get better performance and better CPU Utilization:

![](https://habrastorage.org/webt/ks/6q/mc/ks6qmc7be895eh3y01j0sblzryk.gif)

Also possible to run as server:

```
llama-server.exe -m DeepSeek-R1.Q8_0-00001-of-00015.gguf --port 8082
```

The you can use web interface

![](https://habrastorage.org/r/w1560/webt/j4/qj/xg/j4qjxg4gicuvjkc8aa4j-s0fgce.png)

Enjoy!

