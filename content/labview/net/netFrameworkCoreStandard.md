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
  - .NET
  - DLL
programming_languages:
  - LabVIEW
  - C#
draft: false
---
Many beginner users may be frustrated with .net and how it used in LabVIEW as well as why .NET Core and .NET Framework exists and what the differences. Since last LabVIEW 2024 supported both Core and Standard below is "demystification" with practical examples and some notes. 
<!--more-->
Assumed basic knowledge of .net is present.

In the beginning, there was the .NET Framework. Over time, the .NET Framework struggled to stay competitive, burdened by the weight of backward compatibility. Other languages and frameworks worked on any platform instead of being limited to Windows.
To start fresh, the .NET team created a new version called .NET Core that incorporated
a lot of new ideas: designing and building in the open, working on any platform, focusing on web performance, and so on.

#### .NET Framework and .NET Core

"**.NET Framework**" was the first .NET, started in February 2006 and is Windows-only. Microsoft has stopped adding new features to this. They are going to keep supporting it and will fix security issues, but they don't really want people to keep using it if they have a choice between Framework and Core, so it is more or less "legacy" stuff.

"**.NET Core**" was developed by Microsoft 10 years later, in June 2016. MS wanted a cross-platform runtime, as well as solving some architectural "issues", increase performance. It is the most modern Microsoft-developed .NET and the one they recommend for new projects going forwards. .NET Core 3.1 was the last time .NET Framework had a feature release along side Core's release. Beginning with .NET 5.0, Microsoft .NET Core has been re-branded as .NET (take a note - Microsoft decided to skip version .NET 4 to avoid confusion with .NET Framework 4), starting from this point, it is possible .NET Core may have features that .NET Framework won't.

| Implementation | Included versions                           | Last Update            |
| :------------- | :------------------------------------------ | ---------------------- |
| .NET (Core)    | .NET Core 1.0 - 3.1, .NET 5.0-9.0 and later | v.9.0 - 12th Nov 2024  |
| .NET Framework | .NET Framework 1.0 - 4.8                    | v.4.8.1 — 9th Aug 2022 |

Currently (as per 14th November) active .NET  version is 9.0 - SDK   9.0.100/RunTime 9.0.0 [announced 12th November 2024](https://devblogs.microsoft.com/dotnet/announcing-dotnet-9/) included in [Visual Studio 2022 (17.12)](https://devblogs.microsoft.com/visualstudio/visual-studio-2022-v17-12-with-dotnet-9/) with Language support [C# 13.0](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/language-versioning), [F# 9.0](https://learn.microsoft.com/en-us/dotnet/fsharp/whats-new/fsharp-9) and Visual Basic 16.9

{{% expand ".NET Timeline 2006—2024" %}}
{{% include_md tables.markdown %}}
{{% /expand %}}


Summary: 

|                                 | **.NET Framework**                                           | **.NET Core**                                                |
| ------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| **History**                     | the first implementation of .NET.                            | the latest implementation of .NET.                           |
| **Open Source**                 | No, the only certain components  are open source.            | Yes, .NET Core is open-source.                               |
| **Cross-Platform**              | It works only on the Windows platform.                       | It runs cross-platform like Windows, Linux, and macOS.       |
| **Performance and Scalability** | less effective in comparison to .NET Core in terms of performance and scalability | offers higher performance and scalability compared to .NET Framework. |
| **Command-line Tools**          | heavy for Command Line Interface                             | provides cross platform lightweight command-line tools       |

Let 's consider we have created an application using .NET Framework. After some time, we have decided to create a set of modular applications in .NET Core, put some code to shared libraries. and try to re-use the above same shared library back in .NET Framework. Is it compatible and can we use it? The Answer is No. We cannot use .NET Core Class Library in .NET Framework applications, because of compatibility issues. Basically, the libraries which target to .NET Core can only run in .NET Core-based applications. What is the solution?

### .NET Standard

To make existing libraries port easily between Framework and Core, Microsoft introduced .NET Standard, which defines only the APIs that a .NET implementation needs to fulfill. If you have some projects that are .NET Framework and others that are .NET Core, .NET 5, or later, those projects can use common libraries that target the .NET Standard.

If you want to develop in .NET, forget about Core 1.0-3.1, which are out of support and Framework as well; target the latest long-term support release of .NET (which is 8.0). If you need to support older versions of .NET, you can targeting .NET Standard, which will give your libraries the broadest reach.

So:

```mermaid
graph 
A(.NET Framework) --> |API|B(".NET Standard")
C(.NET Core Framework) --> |API|B
```

---

Well, now from theory we can do some practical exercises.

I will use the latest available Microsoft Visual Studio 2022, then will create three .net Class Libraries, and call both from .NET and LabVIEW environment.

When I'll call ".NET Core" or "Core" I will assume latest and newest .NET. The same with LabVIEW - .NET Core means .-NET

As stated above, there are three ways to create .NET Library - Framework, Core and Standard. An easiest way to create these Libraries is using the Wizard.

I will start with classical .NET Framework:

![image-20241110102542464](assets/image-20241110102542464.png)

On the next page you will configure the project and choose target framework:

![image-20241110105648166](assets/image-20241110105648166.png)

The code created is simple - just one method, which adds two integers:

```c#
namespace NET_Framework_Class_Library
{
    public class NetFrameworkClass
    {
        public string Info => ".NET Framework 4.8.1 Library";
        public int Add(int x, int y) => x + y;
    }
}
```

Take a note that 

```c#
    public int Add(int x, int y) => x + y;
```

is syntax sugar for 

```c#
    public int Add(int x, int y)
    {
        return x + y;
    }
```

This will create DLL, which can be called from LabVIEW like this:

![](assets/netframeeworksnip.png)

So far so good, now I will do the same with Core:

![image-20241110132659816](assets/image-20241110132659816.png)

On the next screen you can select target framework, I will choose .NET 8.0:

![image-20241114090306284](assets/image-20241114090306284.png)

Technically the .NET 9.0 is also available, but LabVIEW currently supported 8.0 only, therefore LTS version was selected.

And the code:

```C#
namespace NET_Core_Class_Library;
	public class NetCoreClass
    {
		public string Info => ".NET 8.0 Library";
		public int Add(int x, int y) => x + y;
    }
```

Take a note, that the code slightly more simply.

Now, before calling this DLL the support is necessary to be enabled in the Options:

![image-20241110160141829](assets/image-20241110160141829.png)

This will enable .NET Core in the Constructor:

![image-20241110160342173](assets/image-20241110160342173.png)

Class needs to be selected:

![image-20241110162220393](assets/image-20241110162220393.png)



In case if you will get message like this:

![image-20241110160850722](assets/image-20241110160850722.png)

Just navigate to [.NET SDKs for Visual Studio](https://dotnet.microsoft.com/en-us/download/visual-studio-sdks), download and install [.NET SDK 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/8.0).

In LabVIEW will be called like this:

![](assets/core_snip.png)

with obviously same result.

By the way, there is other way to create DLL and later console:

```
>mkdir "NET Core Class Library"
>cd "NET Core Class Library"
>dotnet new classlib
```

then modify your class as needed in any text editor (also Notepad is OK, the Visual Studio Code is recommended)

then build or publish:

```
>dotnet build
>dotnet publish
```

Finally I will create .NET Standard Library. Everything the same — starting with this Wizard and the code behind:

![image-20241110141033867](assets/image-20241110141033867.png)



But why do we need Standard at all? To illustrate this I can create two console applications and call these DLLs from both.

The first one is .NET Core — based app. Again with Wizard, selecting appropriate framework:

Now I need to put my Class Library to the dependencies, and call like this:

```c#
Console.WriteLine("Hello, .NET 8.0!");

var core = new NET_Core_Class_Library.NetCoreClass();
Console.WriteLine(core.Info);
Console.WriteLine(core.Add(1, 2));

var standard = new NET_Standard_Class_Library.NetStandardClass();
Console.WriteLine(standard.Info);
Console.WriteLine(standard.Add(3, 4));

var framework = new NET_Framework_Class_Library.NetFrameworkClass();
Console.WriteLine(framework.Info);
Console.WriteLine(framework.Add(5, 6));
```

As you can see, all three Class Libraries can be referenced from .NET Console App and works:

```
>NET Core Console.exe
Hello, .NET!
.NET 8.0 Library
3
.NET Standard 2.1 Library
7
.NET Framework 4.8.1 Library
11
```

So far so good, but now I will create .NET Framework Console App and this attempt will be failed with following message:

![image-20241111102453997](assets/image-20241111102453997.png)

Obviously you can't call .NET 8.0 Class Library from .NET Framework 4.8.1 project as well as .NET Standard 2.1. To be able to load .NET Standard Class Library you should build it with 2.0, then could be loaded:

![image-20241111102819735](assets/image-20241111102819735.png)

Refer to [.NET Standard](https://dotnet.microsoft.com/en-us/platform/dotnet-standard) and [.NET Standard 2.0 Compatibility Chart](https://learn.microsoft.com/en-us/dotnet/standard/net-standard?tabs=net-standard-2-0).

Now can be called:

```c#
using System;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, .NET Framework 4.8.1!");

        //var core = new NET_Core_Class_Library.NetCoreClass();
        //Console.WriteLine(core.Info);
        //Console.WriteLine(core.Add(1, 2));

        var standard = new NET_Standard_Class_Library.NetStandardClass();
        Console.WriteLine(standard.Info);
        Console.WriteLine(standard.Add(3, 4));

        var framework = new NET_Framework_Class_Library.NetFrameworkClass();
        Console.WriteLine(framework.Info);
        Console.WriteLine(framework.Add(5, 6));
    }
}
```

And works:

```
>Net Framework Console.exe
Hello, .NET Framework 4.8.1!
.NET Standard 2.0 Library
7
.NET Framework 4.8.1 Library
11
```

So, you can use .NET Standard 2.0 Class Library in both .NET and .NET Framework Applications.

Now the last question is — can you load .NET Standard 2.x Class Library into LabVIEW?

Yes, you can, and in case if this is .NET Standard 2.0, then it can be loaded as both .NET Framework and .NET Core,  buth selecting Traget as .NET Standard 2.1 will allow to load this as .NET Core only (which is expected behaviour):

![](assets/three_snip.png)

### Troubleshooting

Now, what to do if you have .NET Assembly in your hands, which can't be used in LabVIEW?

In most cases it looks like this:

![image-20241111110019501](assets/image-20241111110019501.png)

In the example above I selected .NET Standard 2.1 Class Library as .NET Framework, and this doesn't work, because not compatible. So, at the beginning you must be sure about target framework used as well as about dependencies.

If you have a very old "legacy" DLL, then you can try to create configuration file, which will allow to load such DLL. This file should have content like this:

```xml
<?xml version ="1.0" encoding="utf-8" ?>
<configuration>
   <startup useLegacyV2RuntimeActivationPolicy="true">
      <supportedRuntime version="v4.0"/>
   </startup>
</configuration> 
```

and saved as LabVIEW.exe.config, placed near LabVIEW.exe. Don't forget to do the same for builded application!

If this not work, then you can try to recompile DLL (kind of reverse engineering).  The source could be done with help of IlSpy (there are may "disassemblers available, but this one is really good). As result you will get Visual Studio Project, where you can change used frameworks. This will work for simple assemblies without "heavy weight" dependencies (we should keep in mind, that different versions of the frameworks could have some "breaking" differences).

To deploy application where .NET is used, the appropriate .NET Run-Time is required, and for .NET Core can be downloaded for free from here.

Actual version compatible with LabVIEW 2024 Q3 as per Nov 2024 is 8.0 LTS, most recent, but not compatible with LabVIEW is .NET 9.0.

If you have "unknown" ,NET DLL in your hands, then the useful free tool is Jet Brains Peek, which allow to get info about header.

### Useful tools for .NET

[Ildasm.exe (IL Disassembler)](https://learn.microsoft.com/en-us/dotnet/framework/tools/ildasm-exe-il-disassembler)

[ILSpy is the open-source .NET assembly browser and decompiler.](https://github.com/icsharpcode/ILSpy)

[dotPeek Free .NET Decompiler and Assembly Browser.](https://www.jetbrains.com/decompiler/)



