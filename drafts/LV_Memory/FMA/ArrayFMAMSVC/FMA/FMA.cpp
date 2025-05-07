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




FMA_API void AddMulUnroll(TD1Hdl a, TD1Hdl b, TD1Hdl c) {
    int32_t len = (*a)->dimSize;
    float* a_data = (*a)->element;
    float* b_data = (*b)->element;
    float* c_data = (*c)->element;

    // Pad array to avoid remainder loop
    int padded_len = (len + 7) & ~7;

    // Main processing with 4x unrolling
    for (int i = 0; i < padded_len; i += 32) { // Process 32 elements/iteration
        __m256 av0 = _mm256_loadu_ps(&a_data[i]);
        __m256 bv0 = _mm256_loadu_ps(&b_data[i]);
        __m256 cv0 = _mm256_loadu_ps(&c_data[i]);
        __m256 res0 = _mm256_fmadd_ps(av0, bv0, cv0);
        _mm256_storeu_ps(&c_data[i], res0);

        // Repeat with offsets 8, 16, 24 for full 32-element unroll
        __m256 av1 = _mm256_loadu_ps(&a_data[i + 8]);
        __m256 bv1 = _mm256_loadu_ps(&b_data[i + 8]);
        __m256 cv1 = _mm256_loadu_ps(&c_data[i + 8]);
        __m256 res1 = _mm256_fmadd_ps(av1, bv1, cv1);
        _mm256_storeu_ps(&c_data[i + 8], res1);

        __m256 av2 = _mm256_loadu_ps(&a_data[i + 16]);
        __m256 bv2 = _mm256_loadu_ps(&b_data[i + 16]);
        __m256 cv2 = _mm256_loadu_ps(&c_data[i + 16]);
        __m256 res2 = _mm256_fmadd_ps(av2, bv2, cv2);
        _mm256_storeu_ps(&c_data[i + 16], res2);

        __m256 av3 = _mm256_loadu_ps(&a_data[i + 24]);
        __m256 bv3 = _mm256_loadu_ps(&b_data[i + 24]);
        __m256 cv3 = _mm256_loadu_ps(&c_data[i + 24]);
        __m256 res3 = _mm256_fmadd_ps(av3, bv3, cv3);
        _mm256_storeu_ps(&c_data[i + 24], res3);
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
