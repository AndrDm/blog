---
title: About some differences between .net Framework, Core and Standard
date: 2024-11-09
authorbox: false
sidebar: false
description: Article about some differences between .net Framework, Core and Standard
categories:
  - LabVIEW
archives:
  - 2024-11
tags:
  - template
draft: true
---
Many beginner users may be frustrated with .net and how it used in LabVIEW as well as why .NET Core and .NET Framework exists and what the differences. Since last LabVIEW 2024 supported both Core and Standard below is "demystification" with practical examples and some notes. 
<!--more-->
Assumed basic knowledge of .net is present.

In the beginning, there was the .NET Framework. Over time, the .NET Framework struggled to stay competitive, burdened by the weight of backward compatibility. Other languages and frameworks worked on any platform instead of being limited to Windows.
To start fresh, the .NET team created a new version called .NET Core that incorporated
a lot of new ideas: designing and building in the open, working on any platform, focusing on web performance, and so on.

#### .NET Framework and .NET Core

"**.NET Framework**" was the first .NET, started in February 2006 and is Windows-only. Microsoft has stopped adding new features to this. They are going to keep supporting it and will fix security issues, but they don't really want people to keep using it if they have a choice between Framework and Core, so it is more or less "legacy" stuff.

"**.NET Core**" was developed by Microsoft 10 years later, in June 2016. MS wanted a cross-platform runtime, as well as solving some architectural "issues", increase performance. It is the most modern Microsoft-developed .NET and the one they recommend for new projects going forwards. .NET Core 3.1 was the last time .NET Framework had a feature release along side Core's release. Beginning with .NET 5.0, Microsoft .NET Core has been re-branded as .NET, starting from this point, it is possible .NET Core may have features that .NET Framework won't.

| Implementation | Included versions                     |
| :------------- | :------------------------------------ |
| .NET (Core)    | .NET Core 1.0 - 3.1, .NET 5 and later |
| .NET Framework | .NET Framework 1.0 - 4.8              |

{{% expand ".NET Timeline" %}}

| Version                  | Start Date    | End Date      |
| :----------------------- | ------------- | ------------- |
| .NET Framework 4.8.1     | 9. Aug. 2022  |               |
| .NET Framework 4.8-4.8.1 | 18. Apr. 2019 |               |
| .NET Framework 4.7-4.7.2 | 11. Apr. 2017 |               |
| .NET Framework 4.6-4.6.2 | 29. Jul. 2015 | 12. Jan. 2027 |
| .NET Framework 4.5-4.5.2 | 9. Oct. 2012  | 26. Apr. 2022 |
| .NET Framework 4.0       | 12. Apr. 2010 | 12. Jan. 2016 |
| .NET Framework 3.5 SP1   | 19. Nov. 2007 | 9. Jan. 2029  |
| .NET Framework 3.0       | 21. Nov. 2006 | 12. Jul. 2011 |
| .NET Framework 2.0       | 17. Feb. 2006 | 12. Jul. 2011 |

| Version                 | Start Date   | End Date     |
| :---------------------- | ------------ | ------------ |
| .NET 8 (LTS)            | Nov 14, 2023 | Nov 10, 2026 |
| .NET 7                  | Nov 8, 2022  | May 14, 2024 |
| .NET 6.0 (LTS)          | Nov 8, 2021  | Nov 12, 2024 |
| .NET 5.0                | Nov 10, 2020 | May 10, 2022 |
| .NET Core 3.0-3.1(LTS)  | Sep 23, 2019 | Dec 13, 2022 |
| .NET Core 2.0-2.1 (LTS) | Aug 14, 2017 | Oct 1, 2018  |
| .NET Core 1.0-1.1       | Jun 27, 2016 | Jun 27, 2019 |

{{% /expand %}}

### NET Core  

|                                 | **.NET Framework**                                           | **.NET Core**                                                |
| ------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| **History**                     | the first implementation of .NET.                            | the latest implementation of .NET.                           |
| **Open Source**                 | No, the only certain components  are open source.            | Yes, .NET Core is open-source.                               |
| **Cross-Platform**              | It works only on the Windows platform.                       | It runs cross-platform like Windows, Linux, and macOS.       |
| **Performance and Scalability** | less effective in comparison to .NET Core in terms of performance and scalability | offers higher performance and scalability compared to .NET Framework. |
| **Command-line Tools**          | heavy for Command Line Interface                             | provides cross platform lightweight command-line tools       |

###  .NET standard

Let 's consider we have created an application using .NET Framework. After some time, we have decided to create a set of modular applications in .NET Core, put some code to shared libraries. and try to re-use the above same shared library back in .NET Framework. Is it compatible and can we use it? The Answer is No. We cannot use .NET Core Class Library in .NET Framework applications, because of compatibility issues. Basically, the libraries which target to .NET Core can only run in .NET Core-based applications.

## What is the solution?

### What happened to Core, Framework, and Standard?


To make existing libraries port easily between Framework and Core, Microsoft introduced .NET Standard, which defines only the APIs that a .NET implementation needs
to fulfill. If you have some projects that are .NET Framework and others that are .NET
Core, .NET 5, or later, those projects can use common libraries that target the .NET
Standard.

If you want to develop in .NET, forget about Core and Framework; target the latest long-term support release of .NET. If you need to support older versions of .NET, you can targeting .NET Standard, which will give your libraries the broadest reach.

---

https://learn.microsoft.com/en-us/lifecycle/products/microsoft-net-and-net-core

https://learn.microsoft.com/de-de/lifecycle/products/microsoft-net-framework

https://devblogs.microsoft.com/dotnet/net-framework-4-5-2-4-6-4-6-1-will-reach-end-of-support-on-april-26-2022/

https://www.google.com/search?q=DIFFERENCES+BETWEEN+%3ANET+CORE+AND+FRAMEWORK&sourceid=chrome&ie=UTF-8

https://www.microfocus.com/documentation/visual-cobol/vc70/VS2017/GUID-3EF28F11-9A12-4E80-9BF7-A16D25626C64.html

https://www.reddit.com/r/csharp/comments/11oom0f/net_framework_vs_net_core/?rdt=64196

https://learn.microsoft.com/en-us/dotnet/standard/choosing-core-framework-server

https://www.c-sharpcorner.com/article/net-framework-vs-net-core-vs-net-standard2/#:~:text=NET%20Framework%20is%20Windows-based,NET%20platform.

https://learn.microsoft.com/en-us/archive/msdn-magazine/2017/september/net-standard-demystifying-net-core-and-net-standard

https://stackoverflow.com/questions/76748330/differences-between-net-vs-net-core-vs-net-standard-vs-net-framework-and-res

https://code-maze.com/differences-between-net-framework-net-core-and-net-standard/

