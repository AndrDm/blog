---
title: Far Manager Crash with ConEmu
date: 2026-04-10
authorbox: false
sidebar: false
description: Far Manager Editor Crash with ConEmu
categories:
  - Knowledge Base
archives: 2026-04
tags:
  - ConEmu
  - Far
draft: false
---
Recent releases of FAR (6644 and newer) crashed with ConEmu in attempt to start editor (<kbd>F4</kbd>). Below is detailed info and how to fix.
<!--more-->

### Detailed report:

Far Manager 3.0.6644, x64

Win 11 Enterprise LTSC 24H2 Build 26100.8117

The latest version of ConEmu, 230724

It works with 3.0.6600, fails with 3.0.6644 (or 6632?) and newer, still present in 6666

### Steps to reproduce

- Install FAR (3.0.6644 or newer)
- Install ConEmu (https://github.com/ConEmu/ConEmu/releases/tag/v23.07.24)
- Open a file in editor (<kbd>F4</kbd>)
- Try navigating down (Down Arrow, or Page Down, of End). 

The console (with FAR in it) just "disappears"

### Explanation

The crash started with commit [2acff8e8e](https://github.com/FarGroup/FarManager/commit/2acff8e8eeaaf5c732dc482e15a89953c9462f37), the addition of new fields in EditorInfo: WindowArea and ClientArea.

It happens in [far/fileedit.cpp, line 2650](https://github.com/mihnita/FarManager/blob/master/far/fileedit.cpp#L2650):

```
	const auto result = m_editor->EditorControl(Command, Param1, Param2);
	if (result&&ECTL_GETINFO==Command)
	{
		const auto Info=static_cast<EditorInfo*>(Param2);
		if (m_bAddSignature)
			Info->Options|=EOPT_BOM;
		if (Global->Opt->EdOpt.ShowTitleBar)
			Info->Options|=EOPT_SHOWTITLEBAR;
		if (Global->Opt->EdOpt.ShowKeyBar)
			Info->Options|=EOPT_SHOWKEYBAR;
		if (CheckStructSize(Info, &EditorInfo::WindowArea))
			Info->WindowArea = GetPosition().as<RECT>();  // !!! HERE
	}
```



### Fixes

Install night build above 6669 ([6675](https://farmanager.com/nightly/Far30b6675.x64.20260410.msi) seems to be OK)

Or install [ConEmu.x64.dll](https://github.com/user-attachments/files/24778302/Plugins.Conemu.zip) to "C:\Program Files\ConEmu\Plugins\ConEmu"

### Links

ConEmu: [Plugin ConEmu crashes Far manager since build 6632 #2630](https://github.com/ConEmu/ConEmu/issues/2630)

FarManager: [Strange crashes while editing files #1080](https://github.com/FarGroup/FarManager/issues/1080)

FarManager: [Recent releases of FAR (6644 and newer) crash with ConEmu #1086](https://github.com/FarGroup/FarManager/issues/1086)

[Far Manager - Ru.Board Thread (Russian)](https://forum.ru-board.com/topic.cgi?forum=5&topic=51024&start=3040#8)

ConEmu: [FAR Plugins: Fix crash on editor open #2631 (ConEmu.x64.dll Fix Here)](https://github.com/ConEmu/ConEmu/pull/2631)

[FarManager GitHub Repository](https://github.com/FarGroup/FarManager)

[ConEmu GitHub Repository](https://github.com/ConEmu/ConEmu)

[FarManager Downloads](https://farmanager.com/download.php?l=ru) / [Files](https://farmanager.com/files.php)

### Changelog

<details>
  <summary>6600-6675</summary>


https://raw.githubusercontent.com/FarGroup/FarManager/master/far/changelog

  ```bash
  --------------------------------------------------------------------------------
  drkns 2026-04-07 18:59:21+01:00 - build 6675
  
  1. Improve keyboard layouts detection.
  
  2. Refactoring.
  
  --------------------------------------------------------------------------------
  drkns 2026-04-07 00:05:18+01:00 - build 6674
  
  1. Logging improvements.
  
  --------------------------------------------------------------------------------
  drkns 2026-04-06 19:36:40+01:00 - build 6673
  
  1. Refactoring.
  
  2. Allow view/edit times as UTC in Attributes and Filters.
  
  --------------------------------------------------------------------------------
  drkns 2026-04-04 23:58:07+01:00 - build 6672
  
  1. Correction of 6070.
  
  --------------------------------------------------------------------------------
  drkns 2026-04-02 20:06:51+01:00 - build 6671
  
  1. Correction of 1.70.1748 and 3.0.4859.
  
  2. Propagate current directory to the terminal.
  
  3. Refactoring.
  
  4. Do not refresh viewer if there are no changes.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-29 20:19:23+01:00 - build 6670
  
  1. Show build date in far:about.
  
  2. Fix setting date/time in corner cases.
  
  3. Make plugins sorting more predictable.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-27 20:33:11+00:00 - build 6669
  
  1. Add upper limit to API struct size checks to detect uninitialized memory.
     This means that some buggy plugins might suddenly stop working. C'est la vie.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-26 20:31:07+00:00 - build 6668
  
  1. Continue 6667.
  
  2. Add paging to 'set' and 'far::about' outputs.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-25 20:07:10+00:00 - build 6667
  
  1. Add sqlite3.dll version check.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-23 23:23:23+00:00 - build 6666
  
  1. Allow decimal size suffixes in filters and search.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-23 22:34:47+00:00 - build 6665
  
  1. Refactoring, tests.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-21 23:34:45+00:00 - build 6664
  
  1. Correction of 6663.2.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-20 21:54:19+00:00 - build 6663
  
  1. Correction of 6659.
  
  2. Continue 6662.
  
  3. Fix mouse handling in command line.
  
  4. Fix mode indicators.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-17 18:15:36+00:00 - build 6662
  
  1. Correction of 6483.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-14 18:40:14+00:00 - build 6661
  
  1. Clarify plugin unloading logic in corner cases.
  
  --------------------------------------------------------------------------------
  drkns 2026-03-13 00:23:35+00:00 - build 6660
  
  1. Color handling improvements.
  
  ```

</details>
