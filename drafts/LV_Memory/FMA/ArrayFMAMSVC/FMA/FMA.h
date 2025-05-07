// The following ifdef block is the standard way of creating macros which make exporting
// from a DLL simpler. All files within this DLL are compiled with the FMA_EXPORTS
// symbol defined on the command line. This symbol should not be defined on any project
// that uses this DLL. This way any other project whose source files include this file see
// FMA_API functions as being imported from a DLL, whereas this DLL sees symbols
// defined with this macro as being exported.
#ifdef FMA_EXPORTS
#define FMA_API __declspec(dllexport)
#else
#define FMA_API __declspec(dllimport)
#endif

// This class is exported from the dll
class FMA_API CFMA {
public:
	CFMA(void);
	// TODO: add your methods here.
};

extern FMA_API int nFMA;

FMA_API int fnFMA(void);
