---
title: LabVIEW Controls Parts IDs
date: 2024-05-08
authorbox: false
sidebar: false
description: List of LabVIEW Controls Parts IDs.
categories:
  - Programming
archives:
  - 2024-05
tags:
  - LabVIEW
draft: false
---
Every LabVIEW's control part contains a "part ID" field which determines its purpose and functionality in the control. For example, a digital numeric control contains two BigMultiCosmetic parts for the increment and decrement buttons; the part ID is what determines which one is which. The Parts Window in the control editor displays the part ID as the textual name for the part, though the actual number is only visible in Heap Peek.

<!--more-->

List of Parts IDs:


| Part ID |          Long Name          |          Description           |
| :-----: | :-------------------------: | :----------------------------: |
|    1    |          Cosmetic           |                                |
|    2    |          Increment          |                                |
|    3    |          Decrement          |                                |
|    4    |       Large Increment       |                                |
|    5    |       Large Decrement       |                                |
|    6    |       Pixel Increment       |                                |
|    7    |       Pixel Decrement       |                                |
|    8    |           Housing           |                                |
|    9    |            Frame            |                                |
|   10    |        Numeric Text         |                                |
|   11    |            Text             |                                |
|   12    |          Ring Text          |                                |
|   13    |          Scrollbar          |                                |
|   14    |        Ring Picture         |                                |
|   15    |            Radix            |                                |
|   16    |         Name Label          |                                |
|   17    |            Scale            |                                |
|   18    |           X Scale           |                                |
|   19    |           Y Scale           |                                |
|   20    |      Out of Range Box       |                                |
|   21    |       Boolean Button        |                                |
|   22    |        Boolean Text         | Apparently lost "Size to Text" |
|   23    |  Slider, Needle, or Thumb   |                                |
|   24    |       Set to Default        |                                |
|   25    |         Decoration          |                                |
|   26    |          List Area          |                                |
|   27    |        Scale Marker         |                                |
|   28    |        Content Area         |                                |
|   29    |          DDO Frame          |                                |
|   30    |         Index Frame         |                                |
|   31    |            Fill             |                                |
|   32    |        Graph Legend         |                                |
|   33    |        Graph Palette        |                                |
|   34    |        X Fit Button         |                                |
|   35    |        Y Fit Button         |                                |
|   36    |      X Fit Lock Button      |                                |
|   37    |      Y Fit Lock Button      |                                |
|   38    |         X Scrollbar         |                                |
|   39    |         Y Scrollbar         |                                |
|   40    |         Scale Tick          |                                |
|   41    |         Color Area          |                                |
|   42    |     Palette Background      |                                |
|   43    | Control or Indicator Symbol |                                |
|   44    |      Extra Frame Part       |                                |
|   45    |       Scale Min Tick        |                                |
|   46    |       Pix Map Palette       |                                |
|   47    |        Select Button        |                                |
|   48    |         Text Button         |                                |
|   49    |        Erase Button         |                                |
|   50    |         Pen Button          |                                |
|   51    |        Sucker Button        |                                |
|   52    |        Bucket Button        |                                |
|   53    |         Line Button         |                                |
|   54    |      Rectangle Button       |                                |
|   55    |   Filled Rectangle Button   |                                |
|   56    |         Oval Button         |                                |
|   57    |     Filled Oval Button      |                                |
|   58    |           Pattern           |                                |
|   59    |      Foreground Color       |                                |
|   60    |      Background Color       |                                |
|   61    |    Pix Map Palette Extra    |                                |
|   62    |          Zoom Bar           |                                |
|   63    |     Boolean True Label      |                                |
|   64    |     Boolean False Label     |                                |
|   65    |         Unit Label          |                                |
|   66    |            Annex            |                                |
|   67    |      Old Graph Cursor       |                                |
|   68    |           Z Scale           |                                |
|   69    |         Color Ramp          |                                |
|   70    |      Output Indicator       |                                |
|   71    |     X Scale Unit Label      |                                |
|   72    |     Y Scale Unit Label      |                                |
|   73    |     Z Scale Unit Label      |                                |
|   74    |       Graph Move Tool       |                                |
|   75    |       Graph Zoom Tool       |                                |
|   76    |      Graph Cursor Tool      |                                |
|   77    |       Graph X Format        |                                |
|   78    |       Graph Y Format        |                                |
|   79    |      Combo Box Button       |                                |
|   80    |     Diagram Identifier      |                                |
|   81    |      Menu Title Label       |                                |
|   82    |           Caption           |                                |
|   83    |        Refnum Symbol        |                                |
|   85    |       Formerly Annex2       |                                |
|   86    |        Boolean Light        |                                |
|   87    |        Boolean Glyph        |                                |
|   88    |        Boolean Divot        |                                |
|   89    |       Boolean Shadow        |                                |
|   90    |             Tab             |                                |
|   91    |      Page List Button       |                                |
|   92    |         Tab Caption         |                                |
|   93    |       Tab Background        |                                |
|   94    |         Scale Name          |                                |
|   95    |          Slide Cap          |                                |
|   97    |     Contained data type     |                                |
|   98    |     Position data type      |                                |
|   99    |          Tab Glyph          |                                |
|   100   |            Grid             |                                |
|   101   |          Num Label          |                                |
|   102   |          Split Bar          |                                |
|   103   |      Mutli Y-Scrollbar      |                                |
|   104   |       Graph ViewPort        |                                |
|   105   |         Grab Handle         |                                |
|   106   |     Graph Splitter Bar      |                                |
|   107   |      Graph Legend Area      |                                |
|   108   |   Graph Legend Scrollbar    |                                |
|   109   |     Data Binding Status     |                                |
|   110   |        Ternary Text         |                                |
|   111   |       Ternary Button        |                                |
|   112   |  Multi-Segment Pipe Flange  |                                |
|   113   |  Multi-Segment Pipe Elbow   |                                |
|   114   |   Multi-Segment Pipe Pipe   |                                |
|   115   |     Graph Legend Frame      |                                |
|   116   |     Scene Graph Display     |                                |
|   117   |       Overflow Status       |                                |
|   118   |        Radix Shadow         |                                |
|   119   |       Custom Cosmetic       |                                |
|   120   |       Typedef Corner        |                                |
|  8000   |     Non-Colorable Decal     |                                |
|  8001   |       Digital Display       |                                |
|  8002   |         Array Index         |                                |
|  8003   |        Variant Index        |                                |
|  8004   |       Listbox Display       |                                |
|  8005   |        Data Display         |                                |
|  8006   |        Measure Data         |                                |
|  8007   |          kNotUsed4          |  Apparently not a valid part.  |
|  8008   |         Tree Legend         |                                |
|  8009   |      Color Ramp Array       |                                |
|  8010   |     Type Def's Control      |                                |
|  8011   |       Cursor Buttons        |                                |
|  8012   |         High Color          |                                |
|  8013   |          Low Color          |                                |
|  8014   |        Graph Cursor         |                                |
|  8015   |     Graph Scale-legend      |                                |
|  8015   |            Table            |                                |
|  8016   |      I/O Name Display       |                                |
|  8017   |  Tab Control Page Selector  |                                |
|  8018   |        Browse Button        |                                |
|  8019   |      Graph Plot Legend      |                                |

Each **Control** or **Indicator** in LabVIEW consists of **parts**. **Parts** are the individual elements which make up all [controls](https://labviewwiki.org/wiki/Control) within [Front Panel](https://labviewwiki.org/wiki/Front_Panel) of a [VI](https://labviewwiki.org/wiki/VI).

[Control editor](https://labviewwiki.org/wiki/Control_editor) allows viewing list of parts in a *Control*, and modifying their properties. *Parts* can also be browsed through [Heap Peek](https://labviewwiki.org/wiki/Heap_Peek), and using XML [Heap storage format](https://labviewwiki.org/wiki/Heap_storage_format).

Useful Tools to inspect Parts of the Controls (you need to enable "Debug Mode" in LabVIEW):

```
"LVdebugKeys=True"
```

# Heap Peek

Heap Peek is a debug feature available at least since [LabVIEW 3.0](https://labviewwiki.org/wiki/LabVIEW_3.0). It allows to view internal data structures created for all loaded VI files, including Front Panel Heap and Block Diagram Heap. The feature is intended for use by LV R&D, though users may in some cases be instructed by LV Support to check something there.

## Enabling Heap Peek

The Heap Peek keyboard shortcut is by default disabled in fresh installations of [LabVIEW](https://labviewwiki.org/wiki/LabVIEW). To enable it:

- Close LabVIEW
- Put the key "LVdebugKeys=True" in your [LabVIEW configuration file](https://labviewwiki.org/wiki/LabVIEW_configuration_file)

Then, to trigger the Heap Peek window:

- Open your VI
- Press CTRL+SHIFT+D+H. On Mac, use the command key instead of CTRL. This will open the Heap Peek window.

## Using Heap Peek

The window is divided into 5 sections:

- Loaded Container Files are in top-left,
- Heap View is top-right,
- Structure selection for info pane in middle-left,
- Toolbar at the middle,
- Information pane is at base.

The first section, as for usage flow, is top-left one. Start there, by left clicking to select the specific file and specific heap you want to look at. When the heap is selected, you can display many details on it by clicking rows of either top-right or middle section. These details will show at base, in Information pane.

**Note:** Heap Peek only displays information and does not have any ability to change the objects. It does, however, display the locations in memory where the information is stored, which makes it easy to use external memory editing software for this purpose.

## Loaded Container Files list

The list is divided into Contexts. In each of these, you can see memory location of the item (within LabVIEW process memory), and entries marked *BDHP* or *FPHP*.

- *BDHP* is Block Diagram Heap
- *FPHP* is Front Panel Heap

**Note:** There are [VIs](https://labviewwiki.org/wiki/VI) in LV that are not real VIs (for example, a pseudo VI that is created to represent a Call By Reference to another application instance on another machine), and attempting to do Find to jump to their components can cause problems because those VIs never expect to actually draw themselves.

## Heap View

Heap, for LV, is really a [Front Panel](https://labviewwiki.org/wiki/Front_Panel) or [Block Diagram](https://labviewwiki.org/wiki/Block_Diagram). Despite the name, they don't really have a heap-like structure. Both FP and BD are stored in form of a tree, strongly inspired by [XML format](https://en.wikipedia.org/wiki/XML). The main difference is - the LV Heap format is binary, not text-based.

In *Heap View*, Heaps are displayed in a flattened form, not in its full tree-like glory. Part of the entries show properties of the whole pane, and the rest is [controls](https://labviewwiki.org/wiki/Control), [indicators](https://labviewwiki.org/wiki/Indicator) and other elements added to the pane.

The view for *FPHP* reveals that controls and indicators are composed of several [parts](https://labviewwiki.org/wiki/Control_Parts) - ie. Numeric controls consist of: Label, Frame, Up/Down buttons, Edit field, Radix and Annex. Each [part has partID](https://labviewwiki.org/wiki/Control_Parts#List_of_parts), which is used by LV to identify type of that part.

**Note:** To learn more about LV Heaps, consider [exporting a few to XML](https://labviewwiki.org/wiki/Heap_storage_format), and looking at them there. The flat list visible in *Heap View* will be very helpful when you understand them, but it's not the best way to acquire that understanding.

## Structure selection

Here you can select one of the structures within VI files other than *FPHP*/*BDHP*. These are per-file, not per-heap - so you won't find any difference in them if you select different part of the same file in Loaded Container Files list.

The selectable options are:

- *VI Fields* - lists values of many common settings and flags of the VI file
- *DS Fields* - lists Data Space properties, including Table of QEs and Table of TMEs
- *OH Fields*
- *Stats* - statistics for objects used within Heaps
- *Link Info* - content of all LinkObj lists
- *Names In Memory*
- *IL Code Stream* - Prints Intermediate Language Code used during code compilation process
- *Code Disassembly* - Prints assembly generated by the compiler for given VI file
- *Transform Metrics*
- *UDClass Info*
- *Full OMUDClassPtr List*
- *Available Implementers*

### DS Fields

[Data Space](https://labviewwiki.org/wiki/Data_Space) in an important concept in LV internals. In short, *Data Space* contains filling for all type mapped [data types](https://labviewwiki.org/wiki/Data_type). Be sure to read more on [Data Space](https://labviewwiki.org/wiki/Data_Space) to understand it better.

The *DS Fields* section contains some general information on [Data Space](https://labviewwiki.org/wiki/Data_Space) of the file, and then displays whole mapping of the Data Space (*DSTM*). As of LV2014 it doesn't show values stored in data space, but it shows offsets within said Data Space, and Type Map Entries (**TME**s) with information about which data type is stored there, with all its properties. The *DSTM* data in this section is cut to only show part of Data Space which is used for different purposes than Heap - the complete Data Space is always larger.

## Toolbar

The toolbar consists of several fields.

- Next/prev buttons
- Currently selected item name text
- **U** button
- **F** button - Click on it to find the object selected in Heap View. LabVIEW will switch to the block diagram or front panel and select related object.
- **uo** button - Toggles between sorting the list on the right by UID (the default) or by memory address.

**Note:** A previous author said that few developers really know the capacities of heap peek. This is true... it's a debugging tool, and it gets hacked up to debug whatever anyone needs to debug.

# Ned options

**Ned, the friendly configuration manager** is a debug feature of [LabVIEW](https://labviewwiki.org/wiki/LabVIEW) in form of a window with list of options. The private, internal set of settings available on that list allow changing various aspects of LabVIEW inner mechanisms.

These settings are not saved across LabVIEW sessions, but many of the options are also valid [LabVIEW configuration file](https://labviewwiki.org/wiki/LabVIEW_configuration_file) keys.

## Enabling Ned Options

Like [Heap Peek](https://labviewwiki.org/wiki/Heap_Peek), Ned Options keyboard shortcut is by default disabled in fresh installations of [LabVIEW](https://labviewwiki.org/wiki/LabVIEW). To enable it:

- Close LabVIEW
- Put the key "LVdebugKeys=True" in your [LabVIEW configuration file](https://labviewwiki.org/wiki/LabVIEW_configuration_file)

Then, to trigger the Ned Options window:

- Open your VI
- Press CTRL+SHIFT+D+N. On Mac, use the command key instead of CTRL. This will open the Ned Options window.

## Using Ned Options

Clicking once on a option will switch it, cycling through two or more values.

# Heap storage format

[Front Panel](https://labviewwiki.org/wiki/Front_Panel) and [Block Diagram](https://labviewwiki.org/wiki/Block_Diagram) of a [VI file](https://labviewwiki.org/wiki/VI) are internally called Heaps. Across versions of [LabVIEW](https://labviewwiki.org/wiki/LabVIEW), several storage formats were used for these heaps.

The format used in currently opened *VI file* can be selected in [Ned options](https://labviewwiki.org/wiki/Ned_options). The format used in new *VI files* depends on the version of LabVIEW.

**Note:** Changing the Heap Storage format in Ned may cause your VI to no longer open! Make sure to experiment on scrap copies of files.

## Contents

## Binary format 1

Used in first versions of LabVIEW. Has [Resource ID](https://labviewwiki.org/wiki/Resource_Container#Known_Resources) of *FPHP* and *BDHP*.

## Binary format 2

First update of the format, prepared to handle separation of [Type Descriptors](https://labviewwiki.org/wiki/Type_descriptor) from heap data. Has [Resource ID](https://labviewwiki.org/wiki/Resource_Container#Known_Resources) of *FPHb* and *BDHb*.

## Binary format 3

After this update, all data stored within heap was separated from the main heap tree, dividing heap into two blocks: structure and data. Has [Resource ID](https://labviewwiki.org/wiki/Resource_Container#Known_Resources) of *FPHc* and *BDHc*.

## Verbose Tagged Text

A text based format, created to be easily readable by humans. Has [Resource ID](https://labviewwiki.org/wiki/Resource_Container#Known_Resources) of *FPHT* and *BDHT*.

## XML format

Since the structure of the heap fits XML format perfectly, would be a sin not to add such option. Has [Resource ID](https://labviewwiki.org/wiki/Resource_Container#Known_Resources) of *FPHX* and *BDHX*.

---

This information above taken from [LabVIEW Wiki](https://labviewwiki.org/wiki/Control_Parts).
