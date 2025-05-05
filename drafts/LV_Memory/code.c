/* Call Library source file */

#include "extcode.h"

/* lv_prolog.h and lv_epilog.h set up the correct alignment for LabVIEW data. */
#include "lv_prolog.h"

/* Typedefs */

typedef struct {
	int32_t dimSize;
	uint8_t element[1];
	} TD1;
typedef TD1 **TD1Hdl;

#include "lv_epilog.h"

void funcName(TD1Hdl array);

void funcName(TD1Hdl array)
{

	/* Insert code here */

}

