// The following ifdef block is the standard way of creating macros which make exporting
// from a DLL simpler. All files within this DLL are compiled with the ARRAYINC_EXPORTS
// symbol defined on the command line. This symbol should not be defined on any project
// that uses this DLL. This way any other project whose source files include this file see
// ARRAYINC_API functions as being imported from a DLL, whereas this DLL sees symbols
// defined with this macro as being exported.
#ifdef ARRAYINC_EXPORTS
#define ARRAYINC_API __declspec(dllexport)
#else
#define ARRAYINC_API __declspec(dllimport)
#endif

// This class is exported from the dll
class ARRAYINC_API CArrayInc {
public:
	CArrayInc(void);
	// TODO: add your methods here.
};

extern ARRAYINC_API int nArrayInc;

ARRAYINC_API int fnArrayInc(void);
