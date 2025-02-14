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

![](assets/image-20250205090637918.png)

Memory - 12 x 64 GB = 768 GB:

![](assets/image-20250205090742608.png)

Memory Benchmark (take a note, this is 12-channels memory with around 180 GB/s read - this is important for performance):

![](assets/image-20250205092255850.png)

Video Adapters (will be unused):

![](assets/image-20250205090845219.png)

SSD Drive:

![](assets/image-20250205091725447.png)

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

![](assets/ds4a-OK.gif)

Let try to ask C Source code for Runge-Kutta Method:

![](assets/ds5a.gif)

It is not very fast, around 1.2 tokens per second, but works.

It recommended to turn Hyper threading Off to get better performance and better CPU Utilization:

![](assets/ks6qmc7be895eh3y01j0sblzryk.gif)

Also possible to run as server:

```
llama-server.exe -m DeepSeek-R1.Q8_0-00001-of-00015.gguf --port 8082
```

The you can use web interface:

![](assets/image-20250205152938717.png)



Enjoy!

