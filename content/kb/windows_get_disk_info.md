---
title: How to get Disks Info in Windows
date: 2025-05-20
authorbox: false
sidebar: false
description: List of useful commands, which I often forgot.
categories:
  - Programming
archives:
  - 2025-05
tags:
  - Windows
draft: false
---
Two trivial commands in Power Shell - Get-Disk and Get-Volume
<!--more-->

```
Windows PowerShell
Copyright (C) Microsoft Corporation. All rights reserved.

Install the latest PowerShell for new features and improvements!
https://aka.ms/PSWindows

PS C:\Windows\system32> Get-Disk

Number Friendly Name Serial Number             Total Size Partition
                                                          Style
------ ------------- -------------     ...     ---------- ----------
0      ST2000DM00... WFL8AGZHRTZ...               1.82 TB GPT
1      SAMSUNG MZ... 0025_38BB_41B2             953.87 GB GPT
3      Verbatim P... 00080600U8LW               476.94 GB MBR
2      Generic Ul... T11000009802                     0 B RAW


PS C:\Windows\system32> Get-Volume

DriveLetter FriendlyName FileSystemType DriveType HealthStatus      Size
----------- ------------ -------------- --------- ------------  ... ----
D           DATA         NTFS           Fixed     Healthy        1.82 TB
F           INSTALL      NTFS           Fixed     Healthy      476.93 GB
            Recovery     NTFS           Fixed     Healthy         990 MB
C           OS           NTFS           Fixed     Healthy      952.63 GB
E                        Unknown        Removable Healthy            0 B


PS C:\Windows\system32>
```
