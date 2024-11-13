---
title: How to prevent Windows Updates with Metered Connection
date: 2024-11-13
authorbox: false
sidebar: false
description: How to prevent Windows Updates with Metered Connection
categories:
  - Knowledge Base
archives: 2024-11
tags:
  - Windows
draft: false
---
Sometimes you need to prevent Windows updates and following restarts of the PC (for example during long endurance test while PC Stays connected to Internet). An easiest way to do this - is just set Metered Connection to public interface
<!--more-->
The Metered Connection can be enabled here:
![](assets/Pasted%20image%2020241113090324.png)

Then on Windows Update will still notify you about updates, but they will be not downloaded:

![image-20241113090734607](assets/image-20241113090734607.png)

Until not enabled in Advanced Options:

![image-20241113091954123](assets/image-20241113091954123.png)

You may get "No connection" troubles with updates in NI Package Manager, in this case just turn off metered connection temporarily. 

