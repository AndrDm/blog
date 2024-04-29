---
title: How to setup simple OPC UA Server in B&R AS
date: 2024-04-17
authorbox: false
sidebar: false
description: My Template.
categories:
  - Programming
archives:
  - 2024-04 (Apr)
tags:
  - B&R
  - OPCUA
draft: false
---
How to setup simple B&R OPC UA Server from the scratch.
<!--more-->

In this exercise I'll use latest available AS v.4.12.5.95:

 ![image-20240417160358490](assets/image-20240417160358490.png)



### Start with new Project

![image-20240417160539302](assets/image-20240417160539302.png)



Step by step:

![image-20240417161325716](assets/image-20240417161325716.png)



Everything by default

![image-20240417160908709](assets/image-20240417160908709.png)

Select Standard PC here:

![image-20240417161014197](assets/image-20240417161014197.png)

If you're lucky, then the project is created and online in Run State:

![image-20240417161641067](assets/image-20240417161641067.png)

Add Global variable (later this Variable will be mapped to OPC UA Node):

![image-20240417161927994](assets/image-20240417161927994.png)

Who said that the Globals are bad?!

I'll change name to "Counter" and keep default type, which is USINT (which is uint8).

![image-20240417162303901](assets/image-20240417162303901.png)

Don't confuse with BYTE Type, this is something different in B&R Interprepation!

Now I'll add C Program. Just drag and drop to MyOPC_UA:

![image-20240417162637108](assets/image-20240417162637108.png)

I prefer "All in One" Template. The changes I'll made very simple:

![image-20240417162849317](assets/image-20240417162849317.png)

Whole program:

```c

#include <bur/plctypes.h>

#ifdef _DEFAULT_INCLUDES
	#include <AsDefault.h>
#endif

void _INIT ProgramInit(void)
{
	Counter = 0;
}

void _CYCLIC ProgramCyclic(void)
{
	if (255 == Counter) Counter = 0;
	else Counter++;
}

void _EXIT ProgramExit(void)
{

}

```

Now time for first build:

![image-20240417163119690](assets/image-20240417163119690.png)

and transfer to the target:

![image-20240417163201324](assets/image-20240417163201324.png)

and transfer

![image-20240417163244915](assets/image-20240417163244915.png)

If you're still lucky, you will see running counter:

![image-20240417163402184](assets/image-20240417163402184.png)

![](assets/counter.gif)

The counter incremented once per second, because by default your Program automatically inserted into 1000ms  cycle:

![image-20240417163932374](assets/image-20240417163932374.png)

Now time to turn ON OPC Server, this can be done here:

![image-20240417164047034](assets/image-20240417164047034.png)

It is OFF by default, needs to be switched to ON:

![image-20240417164154890](assets/image-20240417164154890.png)

By default Security Policies will prohibit Authentication without security, I'll allow for my exercises

![image-20240417164328297](assets/image-20240417164328297.png)

Keep rest by defaults:

![image-20240417164457876](assets/image-20240417164457876.png)

Now we must add mapping:

![image-20240417164633151](assets/image-20240417164633151.png)

The variable is available, but disabled:

![image-20240417164740532](assets/image-20240417164740532.png)

You have to enable it:

![image-20240417164817329](assets/image-20240417164817329.png)

and rebuild:

![image-20240417164905872](assets/image-20240417164905872.png)

Now this Node is appeared in UA Expert:

![](assets/ua_expert_counter.gif)

In the Tree located here:

![image-20240417165733887](assets/image-20240417165733887.png)

That is.
