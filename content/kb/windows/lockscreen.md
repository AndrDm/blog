---
title: How to Disable the Lock Screen
date: 2026-02-12
authorbox: false
sidebar: false
description: How to Disable the Lock Screen
categories:
  - Knowledge Base
archives:
  - 2026-02
tags:
  - windows
draft: false
---
How to Disable the Lock Screen

The lock screen isn’t just a pretty picture — it’s also an extra step before you can enter your password or PIN.

Starting with Windows 8, you have to press any key or click the mouse to move past the lock screen. In some cases, this can even slow things down — for example, after waking the system from sleep.

<!--more-->

If speed matters more to you than aesthetics, run this command as an administrator:

```bash
reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\Personalization" /v NoLockScreen /t REG_DWORD /d 1 /f
```

The change takes effect immediately.

In an organization, you can also use the Do not display the lock screen policy ([link](https://gpsearch.azurewebsites.net/#7401)) ✌️
