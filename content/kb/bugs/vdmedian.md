---
title: IMAQ NthOrder crashed in some cases if float image contains NaN values
date: 2025-05-09
authorbox: false
sidebar: false
description: IMAQ NthOrder crashed in some cases if float image contains NaN values
categories:
  - Knowledge Base
archives:
  - 2025-05
tags:
  - Bug
  - VDM
draft: false
---
I just encountered an interesting exception in LabVIEW when the Median Filter was called for a float image containing a NaN value. It works as expected for small images, but I found one that caused a crash.

<!--more-->

I'm not sure if the size or content is the "trigger" for this issue. Anyway, the snippet below:

![](assets/vision_crash.png)

I also tested it in LabVIEW 2018 32-bit, and it crashed as well.
