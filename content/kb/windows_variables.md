---
title: Some useful Windows Variables
date: 2024-05-12
authorbox: false
sidebar: false
description: List of useful Windows Variables Shortcuts, which I often forgot.
categories:
  - Programming
archives:
  - 2024-05
tags:
  - Windows
draft: false
---
Some useful variables like %PROGRAMFILES% or %USERPROFILE%, which I often forgot.
<!--more-->

| Variable         | Path | Comment |
| ---------------- | ---- | ---- |
| %PROGRAMFILES% | C:\Program Files | Programme |
| %PROGRAMFILES(X86)% | C:\Program Files (x86) | Programme (x86) |
| %USERPROFILE% | C:\Users\Andrey | Benutzer\Andrey |
| %APPDATA% | C:\Users\Andrey\AppData\Roaming |      |
| %TEMP% | C:\Users\Andrey\AppData\Local\Temp |      |

They can be obtained with set command

```
> set >vars.txt
```

and edited with following command

```
rundll32.exe sysdm.cpl,EditEnvironmentVariables
```

or using PowerShell

```
[Environment]::GetFolderPath("Desktop")
```

