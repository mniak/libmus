#include <stdio.h>
#include <libmus.h>

void main() {
    PitchStep p = C;
    const char* pname = PitchStep_ToString(p);
    uint8_t pnum = PitchStep_ToNumber(p);
    printf("The pitch %s has value %d\r\n", pname, pnum);
}