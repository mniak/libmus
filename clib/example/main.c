#include "../libmus.h"
#include <stdio.h>

int main() {
    PitchClass pc = PitchClass_New();
    PitchClass_PrintValue(pc);
    return 0;
}