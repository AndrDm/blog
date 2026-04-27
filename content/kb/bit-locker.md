---
title: Bit Locker Key Recovery from Command Line
date: 2026-04-27
authorbox: false
sidebar: false
description: Bit Locker
categories:
  - Knowledge Base
archives:
  - 2026-04
tags:
  - kb
  - BitLocker
draft: false
---
From the administrator command prompt type `manage-bde -protectors -get <drive letter>:` where <drive letter> is the drive letter for the BitLocker protected drive that you want to recover. Note: The ID under a numerical password (this is the key identifier for the drive).
<!--more-->

cmd or PS as Admin, then:

```
manage-bde -protectors -get C:
```

The result:

```powershell
PS C:\Windows\system32> manage-bde -protectors C: -get
BitLocker Drive Encryption: Configuration Tool version 10.0.26100
Copyright (C) 2013 Microsoft Corporation. All rights reserved.

Volume C: [OS]
All Key Protectors

    TPM:
      ID: {EBDEC50E-0666-4674-9EF4-XXXXXXXXXXXX}
      PCR Validation Profile:
        7, 11
        (Uses Secure Boot for integrity validation)

    Numerical Password:
      ID: {4B82A66E-A0C0-4EF4-AB39-XXXXXXXXXXXX}
      Password:
        571406-613470-463529-412599-375045-XXXXXX-XXXXXX-XXXXXX
      Backup type:
        AD backup
        AAD backup
        Printed

PS C:\Windows\system32> manage-bde -protectors D: -get
BitLocker Drive Encryption: Configuration Tool version 10.0.26100
Copyright (C) 2013 Microsoft Corporation. All rights reserved.

Volume D: [DATA]
All Key Protectors

    External Key:
      ID: {B61E42CF-499F-4795-AAA2-XXXXXXXXXXXX}
      External Key File Name:
        B61E42CF-499F-4795-AAA2-XXXXXXXXXXXX.BEK
      Automatic unlock enabled.

    Numerical Password:
      ID: {D4689A81-901A-4151-9F26-XXXXXXXXXXXX}
      Password:
        353639-703813-547536-007909-315403-XXXXXX-XXXXXX-XXXXXX
      Backup type:
        AD backup
        AAD backup
        Printed

PS C:\Windows\system32>
```

It is the same key as obtained by backup:

```
BitLocker Drive Encryption recovery key 

To verify that this is the correct recovery key, compare the start of the following identifier with the identifier value displayed on your PC.

Identifier:

	4B82A66E-A0C0-4EF4-AB39-XXXXXXXXXXXX

If the above identifier matches the one displayed by your PC, then use the following key to unlock your drive.

Recovery Key:

	571406-613470-463529-412599-375045-XXXXXX-XXXXXX-XXXXXX

If the above identifier doesn't match the one displayed by your PC, then this isn't the right key to unlock your drive.
Try another recovery key, or refer to https://go.microsoft.com/fwlink/?LinkID=260589 for additional assistance.
```

