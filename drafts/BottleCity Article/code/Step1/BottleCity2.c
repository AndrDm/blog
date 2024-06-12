//==============================================================================
//
// Title:		BottleCity1
// Purpose:		A short description of the command-line tool.
//==============================================================================

#include <ansi_c.h>

int main() {
    int t = 0, w = 99, i, a, s, b, X, Y, Z, d;
    t *= 60;

    for (w = 99, ++t, i = 6000; i--; ) {
        for (a = (i % w) / 50 - 1, s = b = 1 - i / 4000, X = t, Y = Z = d = 1;
             ++Z < w && (Y < 6 - (32 < Z && 27 < X % w && X / 9 ^ Z / 8) * 8 % 46 || d
                         || (s = (X & Y & Z) % 3 / Z, a = b = 1, d = Z / w)); Y -= b){
            X += a;
        }
		double gray = 1.0 - d * Z / w + s;
		printf("d = %d\n", d);
		printf("Z = %d\n", Z);
		printf("w = %d\n", w);
		printf("s = %d\n", s);
		printf("I = %f @ pix %d\n", gray, i);
		break;
    }
    return 0;
}