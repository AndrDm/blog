EUROASM AutoSegment=Yes, CPU=X64,  SIMD=AVX512, EVEX=ENABLED
incrASM PROGRAM Format=DLL, Width=64, Model=Flat, Subsystem=CON, IconFile=

EXPORT incasm
one dq 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0

incasm PROC
    vmovupd zmm1, [one]
    ; Loop through the array in chunks of 8 elements (AVX512 can handle 8 doubles at a time)
    .loop:
        cmp rdx, 0 ; Check if we've processed all elements
        je .done
        vmovupd zmm0, [rcx] ; Load 8 elements from the array into zmm0
        vaddpd zmm0, zmm0, zmm1 ; Add 1 to each element in zmm0
        vmovupd [rcx], zmm0 ; Store the result back to the array
        add rcx, 64  ; 8 elements * 8 bytes per element
        sub rdx, 8 ; Move to the next 8 elements
        jmp .loop
    .done:
ENDPROC incasm

ENDPROGRAM incrASM
