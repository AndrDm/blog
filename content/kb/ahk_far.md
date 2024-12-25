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

<!--more-->

Slightly more complicated example to solve an issue with positioning in Z-order and multiple copies:

```
#Requires AutoHotkey v2.0

#f::
{
    SetTitleMatchMode ('RegEx')
    if WinWait('- Far', , 1) {
        WinActivate
    }
    else {
        SetWorkingDir("C:\Program Files\ConEmu")
        Run ("C:\Program Files\ConEmu\ConEmu64.exe")
        SetTitleMatchMode ('RegEx')
        WinWait('- Far', , 3)
        WinActivate
    }
}
```

My actual script extended for Typora and for <kbd>Win</kbd>+<kbd>SHIFT</kbd>+<kbd>F</kbd>:

```
#Requires AutoHotkey v2.0

#f::
{
    SetTitleMatchMode ('RegEx')
    if WinWait('- Far', , 1) {
        WinActivate
    }
    else {
        SetWorkingDir("C:\Program Files\ConEmu")
        Run ("C:\Program Files\ConEmu\ConEmu64.exe")
        SetTitleMatchMode ('RegEx')
        WinWait('- Far', , 3)
        WinActivate
    }
}

#+f::
{
    SetWorkingDir("C:\Program Files\ConEmu")
    Run ("C:\Program Files\ConEmu\ConEmu64.exe")
    SetTitleMatchMode ('RegEx')
    WinWait('- Far', , 3)
    WinActivate
}

#w::
{
    SetTitleMatchMode ('RegEx')
    if WinWait('Typora', , 1) {
        WinActivate
    }
    else {
        SetWorkingDir("C:\Users\Andrey\Desktop")
    	Run ("C:\Program Files\Typora\Typora.exe")
        SetTitleMatchMode ('RegEx')
        WinWait('Typora', , 3)
        WinActivate
    }
}

#+w::
{
        SetWorkingDir("C:\Users\Andrey\Desktop")
    	Run ("C:\Program Files\Typora\Typora.exe")
        SetTitleMatchMode ('RegEx')
        WinWait('Typora', , 3)
        WinActivate
}


#+m::
{
    MsgBox ("Message")
    TrayTip("Tip")
}

```

