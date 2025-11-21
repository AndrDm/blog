uint32_t isqrt(uint32_t x) {
    uint32_t res = 0;
    uint32_t bit = 1 << 30;

    while (bit > x) bit >>= 2;

    while (bit) {
        if (x >= res + bit) {
            x -= res + bit;
            res = (res >> 1) + bit;
        } else {
            res >>= 1;
        }
        bit >>= 2;
    }
    return res;
}
