# AmpelBR
Simple Traffic Light based on C++ State Machine running on B&amp;R PLC

In this exercise I'll use latest available AS v.6.0.2.177 (25-Jun-2024):

![image-20241211170910677](assets/image-20241211170910677.png)


### Start with new Project

![image-20241211155419766](assets/image-20241211155419766.png)



Step by step:

![image-20241211155522438](assets/image-20241211155522438.png)



Everything by default

![image-20241211155607024](assets/image-20241211155607024.png)

Select Standard PC here:

![image-20241211155730913](assets/image-20241211155730913.png)

By default it won't to go online automatically:

![image-20241211155957727](assets/image-20241211155957727.png)

So, I have so select ArSim

![image-20241211160049291](assets/image-20241211160049291.png)



Now we are online:

![image-20241211160240617](assets/image-20241211160240617.png)

As first step I'll add C++ Program (I prefer all in one):

![image-20241211160535653](assets/image-20241211160535653.png)



Add some local variables (later some these Variable will be mapped to OPC UA Nodes):

![image-20241211161446677](assets/image-20241211161446677.png)

Duration Variables will hold how long our traffic light will emit the red/yellow/green ligths, boolen variables are just outputs, trace messages will be explained later.

### State Machine

As "Template" I will take [High-Performance Hierarchical Finite State Machine](https://github.com/andrew-gresyk/HFSM2) written by Andrew Gresyk.

It is single header heriarchical FSM framework in C++11, with fully statically-defined structure (no dynamic allocations), built with variadic templates and "compatible" with B&R.

All what we need is just to copy machine.hpp to the project.

Whole main.cpp as simple as following:

```cpp
#include <bur/plctypes.h>

#ifdef _DEFAULT_INCLUDES
	#include <AsDefault.h>
#endif

// amount of memory to be allocated for heap storage must be specified 
// for every ANSI C++ program with the bur_heap_size variable
unsigned long bur_heap_size = 0xFFFF;

#include "machine.hpp"
#include "Logging.hpp"
#include "States.hpp"

//////////////////////////////////////////////////////////////////////////////// 

// shared data storage instance
Context context;
FSM::Instance machine{context};

void _INIT ProgramInit(void)
{
	// Insert code here
	CyclesCount = 0;
	ClearTraceLog();
	DurationRed = 5;
	DurationYellow = 3;
	DurationGreen = 5;
	DurationGreenBlinking = 3;
	LightRed = false;
	LightYellow = false;
	LightGreen = false;
}


void _CYCLIC ProgramCyclic(void)
{
	// Insert code here 
	machine.update();
	CyclesCount++;
	TraceMessage("---cycle---");
}


void _EXIT ProgramExit(void)
{
	// Insert code here 
}

```

The only difference is that the original example requires while loop in the main() function:

```
int main() {
	// shared data storage instance
	Context context;

	FSM::Instance machine{context};

	while (machine.isActive<Off>() == false)
		machine.update();

	return 0;
}
```

But on B&R PLC this high-level loop is replaced by CYCLIC program, therefore not needed - the PLC will call it in the loop again and again.

All states placed into states.cpp

```cpp
#include "machine.hpp"

// data shared between FSM states and outside code
struct Context {
	unsigned cycleCount = 0;
	unsigned countGreenBlinking = 0;
	unsigned countGreen = 0;
	unsigned countYellow = 0;
	unsigned countRed = 0;
};

// convenience typedef
using M = hfsm2::MachineT<hfsm2::Config::ContextT<Context>>;

// macro magic invoked to simplify FSM structure declaration
#define S(s) struct s

// State machine structure
using FSM = M::PeerRoot<
	// sub-machine ..
	M::Composite<S(On),
	// .. with 5 sub-states
	S(Red),
	S(YellowDownwards),
	S(YellowUpwards),
	S(GreenBlinking),
	S(Green)
	>,
				S(Off)
			>;
#undef S

//------------------------------------------------------------------------------

static_assert(FSM::regionId<On>()			  ==  1, "");

static_assert(FSM::stateId<On>()			  ==  1, "");
static_assert(FSM::stateId<Red>()			  ==  2, "");
static_assert(FSM::stateId<YellowDownwards>() ==  3, "");
static_assert(FSM::stateId<YellowUpwards>()	  ==  4, "");
static_assert(FSM::stateId<GreenBlinking>()	  ==  5, "");
static_assert(FSM::stateId<Green>()			  ==  6, "");
static_assert(FSM::stateId<Off>()			  ==  7, "");

////////////////////////////////////////////////////////////////////////////////

// top-level region in the hierarchy
struct On
	: FSM::State // necessary boilerplate!
{
	// called on state entry
	void enter(Control& control) {
		control.context().cycleCount = 0;
		TraceMessage("On");
	}
};

//------------------------------------------------------------------------------

// sub-states
struct Red
	: FSM::State
{
	void enter(Control& control) {
		++control.context().cycleCount;
		LightRed = true;
		LightYellow = false;
		LightGreen = false;
		control.context().countRed = 0;
		CountDown = DurationRed + DurationYellow + 2;
		TraceMessage("Red");
		
	}

	// state can initiate transitions to _any_ other state
	void update(FullControl& control) {

		// multiple transitions can be initiated, can be useful in a hierarchy
		if (control.context().cycleCount > 10)
			control.changeTo<Off>();
		else {
			if (control.context().countRed++ >= DurationRed){
				control.changeTo<YellowDownwards>();
			}
		}
		CountDown--;
	}
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct YellowDownwards
	: FSM::State
{
	void enter(Control& control) {
		TraceMessage("Yellow v");
		LightYellow = true; //Keep Red On
		control.context().countYellow = 0;
	}

	void update(FullControl& control) {
		if (control.context().countYellow++ > DurationYellow) { //was 3
			control.changeTo<Green>();
		}
		CountDown--;
	}
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct YellowUpwards
	: FSM::State
{
	void enter(Control& control) {
		TraceMessage("Yellow ^");
		LightGreen = false;
		LightYellow = true;
		control.context().countYellow = 0;
	}

	void update(FullControl& control) {
		if (control.context().countYellow++ > DurationYellow) {
			control.changeTo<Red>();
		}
	}
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct GreenBlinking
	: FSM::State
{
	void enter(Control& control) {
		TraceMessage("Green Blinking");
		control.context().countGreenBlinking = 0;
	}

	void update(FullControl& control) {
		LightGreen = !LightGreen; // Blinking
		if ((control.context().countGreenBlinking++ > DurationGreenBlinking) && !LightGreen) {
			control.changeTo<YellowUpwards>();
		}
		CountDown--;
	}
};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct Green
	: FSM::State
{
	void enter(Control& control) {
		TraceMessage("Green");
		control.context().countGreen = 0;
		LightRed = false;
		LightYellow = false;
		LightGreen = true;
		CountDown = DurationGreen + DurationGreenBlinking + 4;
	}

	void update(FullControl& control) {
		if (control.context().countGreen++ > DurationGreen){
			control.changeTo<GreenBlinking>();
		}
		CountDown--;
	}
};

//------------------------------------------------------------------------------

// another top-level state
struct Off
	: FSM::State
{
	void enter(Control&) {
		TraceMessage("Off");
	}
};

```

Trace log is very simple:

```cpp
#define MAX_LOG_ENTRIES 200 // ca 16KB Log - Default 200 Entries. 
							// Can be increased, but don't forget to allocate TraceMessages Variable!

void ClearTraceLog()
{
	for (int i=0; i<MAX_LOG_ENTRIES; i++) strcpy(TraceMessages[i], "");
}

void TraceMessage(const char* Message)
{
	for (int i=0; i<MAX_LOG_ENTRIES; i++){
		strcpy(TraceMessages[i], TraceMessages[i+1]);
	}
	strcpy(TraceMessages[MAX_LOG_ENTRIES], Message);
}
```

This is how it works:

![](assets/plc_log.gif)

Very simple.

### OPC UA

To get traffic light visualized I will map variables to OPC UA Server.

In AS 6 the setings for OPC UA was moved to the following location:

![image-20241211164729090](assets/image-20241211164729090.png)

I will enable everything. Strange, I wasn't able to get connected to the OPC UA anonymously, therefore added one user here:

![image-20241211164904784](assets/image-20241211164904784.png)

Second part of the Config:

![image-20241211165004289](assets/image-20241211165004289.png)

Mapping is trivial:

![image-20241211165036609](assets/image-20241211165036609.png)

(you have to add DefaultView first to get this in the Project).

These appeared now in UA Expert:

![](assets/uaexpert.gif)

Finally I will create simple LabVIEW VI using OPC UA Toolkit to get simple SCADA:

![](assets/TrafficLight.png)

This is how it works:

![](assets/tl1.gif)

Update 12-Dec-2024 — simple mappView Visualization added:

![](assets/ampel2.gif)

Happy coding!
