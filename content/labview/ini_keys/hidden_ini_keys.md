---
title: "LabVIEW's hidden INI Keys"
date: 2024-04-29
lastmod: 2024-04-30
authorbox: false
sidebar: false
toc: false
description: List of hidden LabVIEW's INI Keys.
categories:
  - "LabVIEW"
archives:
  - "2024-04"
tags:
  - "LabVIEW"
  - "INI"
draft: false
---
There are some "hidden" INI keys available in LabVIEW, which I often forgot
<!--more-->

## Run At Any Loop

```
showRunAtAnyLoopMenuItem=True
```

Allow to open references and run not only in Root Loop, but in any loop.

Refer to [Re: What Execution System does LabVIEW use with VI server?](https://forums.ni.com/t5/LabVIEW/What-Execution-System-does-LabVIEW-use-with-VI-server/m-p/4435119#M1308630) and [The Root Loop](http://www.labviewcraftsmen.com/blog/the-root-loop)

## Maximal amount of threads in a parallel for loop

```
ParallelLoop.MaxNumLoopInstances=256
```

## Save for Previous Version
Introduced in LabVIEW 2024 and finally released in LabVIEW 2025 (so, no need to set any longer manually) 

```ini
FeatureToggle.Editor.ProjectSaveVersion=True
```

This feature allows you to use LabVIEW 2024 Q1 to edit VIs that can be opened with older versions of the LabVIEW editor. This allows you to collaborate on a project with others who haven’t upgraded to the same version of LabVIEW that you are using. Refer to [Preview Feature in LabVIEW 2024 Q1: Developing a LabVIEW Project in an Older Save Version File Format ](https://forums.ni.com/t5/LabVIEW-Public-Beta-Program-in/Preview-Feature-in-LabVIEW-2024-Q1-Developing-a-LabVIEW-Project/m-p/4350547)

## Enabling Heap Peek and Ned

```ini
LVdebugKeys=True
```

Heap Peek is a debug feature which allows to view internal data structures created for all loaded VI files, including Front Panel Heap and Block Diagram Heap. The feature is intended for use by LV R&D.

 to trigger the Heap Peek window:

- Open your VI
- Press CTRL+SHIFT+D+H.

to trigger the Ned Options Window

- Press CTRL+SHIFT+D+N.

Refer to [LabVIEW Wiki - Heap Peek](https://labviewwiki.org/wiki/Heap_Peek) & [Lava - Heap Peek](https://lavag.org/topic/12500-heap-peek-and-other-internal-debug-tools/).

## Additional Options for Control's Customization

```ini
enableSecretPopups=True
```

Christina talks about it a bit [here on LAVA](https://lavag.org/topic/18469-i-found-some-more-hidden-ini-keys/?do=findComment&comment=110810). This will add new menu Items to popup for things like centering text, or having the boolean decal grow with the button resize.

Links: [1](https://forums.ni.com/t5/LabVIEW/Boolean-text-Size-to-text-the-boolean-text-size-fixed-in-my/m-p/4371342#M1284794), [2](https://lavag.org/topic/18469-i-found-some-more-hidden-ini-keys/#comment-110810), [3](https://forums.ni.com/t5/Past-NIWeek-Sessions/Customizing-NI-LabVIEW-Controls-and-Indicators/ta-p/3497319).

## Additional Private Property Nodes and Methods

```ini
SuperSecretPrivateSpecialStuff=True
```

## Enable scripting (available in VI Server)

```ini
server.viscripting.showScriptingOperationsInEditor=True
```

---

Some additional Info: [LabVIEW configuration file/Miscellaneous](https://labviewwiki.org/wiki/LabVIEW_configuration_file/Miscellaneous).

