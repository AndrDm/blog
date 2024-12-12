---
title: Autohotkey script to run Far Manager from ConEmu
date: 2024-12-12
authorbox: false
sidebar: false
description: Autohotkey script to run Far Manager from ConEmu
categories:
  - Knowledge Base
archives: 2024-12
tags:
  - AHK
  - Far
draft: true
---

It is necessary to set Working Directory, otherwise ConEmu will not start Far Manager.
The script for Win+F:

```
#Requires AutoHotkey v2.0

#f::
{
    SetWorkingDir("C:\Program Files\ConEmu")
    Run ("C:\Program Files\ConEmu\ConEmu64.exe")
}
```

