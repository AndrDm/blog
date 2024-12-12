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
draft: false
---

It is necessary to set Working Directory, otherwise ConEmu will not start Far Manager.

The [AutoHotkey](https://www.autohotkey.com) script for <kbd>Win</kbd>+<kbd>F</kbd>:

```
#Requires AutoHotkey v2.0

#f::
{
    SetWorkingDir("C:\Program Files\ConEmu")
    Run ("C:\Program Files\ConEmu\ConEmu64.exe")
}
```

