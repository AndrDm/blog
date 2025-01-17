---
title: How to use more than 64 logical processors in a parallel for loop?
date: 2025-01-17
authorbox: false
sidebar: false
description: Two methods
categories:
  - Programming
archives:
  - 2025-01
keywords: 
tags: 
draft: false
programming_languages:
  - LabVIEW
---
By default its limited, but can be manually increased in LabVIEW.ini

```
ParallelLoop.MaxNumLoopInstances=256
```