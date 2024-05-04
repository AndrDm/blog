---
title: How to determine which decimal separator used
date: 2024-05-04
authorbox: false
sidebar: false
description: 
categories:
  - Programming
archives:
  - 2024-05
tags:
  - LabVIEW
draft: false
---
Playing around with Signal Processing Toolkit, found funny method to detect which Decimal Separator used in OS — just perform conversion of the string "1,23" to double  and compare to double constant "1,23".
<!--more-->
![image-20240504112347984](assets/image-20240504112347984.png)

More information on NI Forum: ## [Read Decimal Symbol "," or "."](https://forums.ni.com/t5/LabVIEW/Read-Decimal-Symbol-quot-quot-or-quot-quot/m-p/4193952)

And some variations:

![](assets/DecimalSeparator.png)

or this:

![](assets/rdx.png)

or more simple:

![](assets/Detectcomma(1).png)

or more hard, but tecnically accurate way using WinAPI:

![](decimal_sep.png)
