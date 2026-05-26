---
title: How to get CPU Info
date: 2026-05-25
description: Code size
programming_languages:
  - Rust
categories:
  - Programming
archives:
  - 2026-05
tags:
  - Hardware
  - CPU
draft: false
---

An easiest way — from registry.

<!--more-->

```
fn collect_cpu_name_from_registry() -> Option<String> {
    read_registry_string(
        "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
        "ProcessorNameString",
    )
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
}
```

Registry:

![image-20260525072923658](assets/image-20260525072923658.png)

BIOS

![image-20260525073007267](assets/image-20260525073007267.png)

Originally from https://github.com/TX230/winproc-tui

