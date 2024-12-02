---
title: VMWare Workstation — Performance tuning
date: 2024-12-02
authorbox: false
sidebar: false
description: How to increase performance of VMWare Workstation on Win 11 24H2
categories:
  - Knowledge Base
archives:
  - 2024-12
tags:
  - VMWare
draft: false
---
I was experiencing extremely poor performance of the VMWare Workstation on a Xeon PC running Windows 11 LTSC 24H2. Below the command with some magic.

<!--more-->

The command which was very helpful is the following:

```
powercfg /powerthrottling disable /path "C:\Program Files (x86)\VMware\VMware Workstation\x64\vmware-vmx.exe"
```

<!--more-->
Originally from [Vmware 17 Pro very slow on Windows 11 22H2](https://community.broadcom.com/vmware-cloud-foundation/discussion/vmware-17-pro-very-slow-on-windows-11-22h2).
