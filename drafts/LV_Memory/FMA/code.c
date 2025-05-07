/* Call Library source file */

#include "extcode.h"

/* lv_prolog.h and lv_epilog.h set up the correct alignment for LabVIEW data. */
#include "lv_prolog.h"

/* Typedefs */

typedef struct {
	int32_t dimSize;
	float element[1];
	} TD1;
typedef TD1 **TD1Hdl;

#include "lv_epilog.h"

void funcName(TD1Hdl a, TD1Hdl b, TD1Hdl c);

void funcName(TD1Hdl a, TD1Hdl b, TD1Hdl c)
{

	/* Insert code here */

}

