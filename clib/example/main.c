#include "../libmus.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    PitchClass pc = NewPitchClass();

    char* fullName = PitchClass_PrettyName(pc);
    printf("The full name of the pitch class is %s", fullName);
    free(fullName);
    return 0;
}