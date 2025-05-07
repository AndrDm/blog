// FMA.cpp : Defines the exported functions for the DLL.
//

#include "framework.h"
#include "FMA.h"

#include <immintrin.h>

/* Call Library source file */

#include "extcode.h"

/* lv_prolog.h and lv_epilog.h set up the correct alignment for LabVIEW data. */
#include "lv_prolog.h"

/* Typedefs */

typedef struct {
    int32_t dimSize;
    float element[1];
} TD1;
typedef TD1** TD1Hdl;

#include "lv_epilog.h"


FMA_API void AddMul(TD1Hdl a, TD1Hdl b, TD1Hdl c) {
    int32_t len = (*a)->dimSize;
    float* a_data = (*a)->element;
    float* b_data = (*b)->element;
    float* c_data = (*c)->element;

    // Process 8 elements/iteration using AVX2
    for (int i = 0; i <= len - 8; i += 8) {
        __m256 av = _mm256_loadu_ps(&a_data[i]);
        __m256 bv = _mm256_loadu_ps(&b_data[i]);
        __m256 cv = _mm256_loadu_ps(&c_data[i]);
        __m256 res = _mm256_fmadd_ps(av, bv, cv);
        _mm256_storeu_ps(&c_data[i], res);
    }

    // Handle remaining elements
    for (int i = len - (len % 8); i < len; i++) {
        c_data[i] = a_data[i] * b_data[i] + c_data[i];
    }
}


// This is an example of an exported variable
FMA_API int nFMA=0;

// This is an example of an exported function.
FMA_API int fnFMA(void)
{
    return 0;
}

// This is the constructor of a class that has been exported.
CFMA::CFMA()
{
    return;
}
