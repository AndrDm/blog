//==============================================================================
//
// Title:		ArrayInc
// Purpose:		A short description of the library.
//
// Created on:	05.05.2025 at 06:18:41 by Andrey Dmitriev.
// Copyright:	GE Inspection Technologies GmbH. All Rights Reserved.
//
//==============================================================================

//==============================================================================
// Include files

#include <utility.h>
#include <immintrin.h>
#include <stdint.h>

/* Call Library source file */

#include "C:\Program Files\National Instruments\LabVIEW 2025\cintools\extcode.h"

/* lv_prolog.h and lv_epilog.h set up the correct alignment for LabVIEW data. */
#include "C:\Program Files\National Instruments\LabVIEW 2025\cintools\lv_prolog.h"

/* Typedefs */

typedef struct {
	int32_t dimSize;
	uint8_t element[1];
	} TD1;
typedef TD1 **TD1Hdl;

#include "C:\Program Files\National Instruments\LabVIEW 2025\cintools\lv_epilog.h"

#include "ArrayInc.h"

void funcNameInc(TD1Hdl array)
{
    if (!array || !*array) return;  // Handle null input
    
    TD1 *arr = *array;
    uint8_t *data = arr->element;
    const int32_t size = arr->dimSize;
    
    // AVX2 vectorized processing (32 elements at a time)
    const __m256i ones = _mm256_set1_epi8(1);
    int i = 0;
    
    // Process 32-byte chunks using AVX2
    for (; i <= size - 32; i += 32) {
        __m256i vec = _mm256_loadu_si256((__m256i*)(data + i));
        vec = _mm256_add_epi8(vec, ones);
        _mm256_storeu_si256((__m256i*)(data + i), vec);
    }

    // Process remaining elements
    for (; i < size; i++) {
        data[i]++;
    }
}
//==============================================================================
// DLL main entry-point functions

int __stdcall DllMain (HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
	switch (fdwReason) {
		case DLL_PROCESS_ATTACH:
			if (InitCVIRTE (hinstDLL, 0, 0) == 0)
				return 0;	  /* out of memory */
			break;
		case DLL_PROCESS_DETACH:
			CloseCVIRTE ();
			break;
	}
	
	return 1;
}
