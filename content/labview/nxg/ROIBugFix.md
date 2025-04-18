---
title: How to fix annoying Bug with Rotated ROI in LabVIEW NXG
date: 2025-04-18
sidebar: false
authorbox: false
description: My Template.
categories:
  - LabVIEW
archives:
  - 2025-04
tags:
  - NXG
  - Bug
draft: false
---
Annoying issue in LabVIEW NXG/Vision with **Rotated ROI**: When an ROI is rotated and the image is subsequently zoomed in/out by the user, the anchors remain unrotated. They only update after moving or resizing the ROI. Here’s how to fix it

<!--more-->

LabVIEW NXG 5.1.0; Build 8.4.0.49575.

Vision Development Module 2025 Q2.

Code:

![](assets/Screenshot 2025-04-18 08.56.59.png)

And the issue:

![](assets/bug.gif)

To get this fixed you can use dnSpy software (and the fact that LabVIEW NXG including Vision based on .net, and the code was not obfuscated much).

All what you need to modify this file a little bit:

```
C:\Program Files\National Instruments

\LabVIEW NXG 5.0\Addons\NI\Vision\Core-src\20.6.0\NationalInstruments.VisionCommon.Display.Controls.dll

\Shared\LabVIEW NXG Run-Time\5.0\NationalInstruments.VisionCommon.Display.Controls.dll
```

On Zoom Level Change the following happened:

![image-20250418090648166](assets/image-20250418090648166.png)

You have to change this Method:

![image-20250418091839188](assets/image-20250418091839188.png)

All what you need is to add 4 lines code 

```c#
			position = ROICalculations.RotatePoint(position, point, rotatedRectangleROI.RotationAngle);
			position2 = ROICalculations.RotatePoint(position2, point, rotatedRectangleROI.RotationAngle);
			position3 = ROICalculations.RotatePoint(position3, point, rotatedRectangleROI.RotationAngle);
			position4 = ROICalculations.RotatePoint(position4, point, rotatedRectangleROI.RotationAngle);
```

right here:

![image-20250418091234189](assets/image-20250418091234189.png)

After compiling and saving (ensure write permissions in the `Program Files` folder):

![](assets/bug-fix.gif)

Used Tool - [dnSpy 6.1.8](https://github.com/dnSpy/dnSpy).

[ILSpy](https://github.com/icsharpcode/ILSpy) is also useful, but I wasn’t able to get functional assembly after rebuild.
