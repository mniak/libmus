#include "../libmus.h"
#include <stdio.h>
#include <stdlib.h>

int main()
{
    PitchClass pc = NewPitchClass();

    char* fullName = PitchClass_PrettyName(pc);
    printf("Name %s <- default value\n", fullName);
    free(fullName);

    PitchClass_SetStep(pc, 2);
    fullName = PitchClass_PrettyName(pc);
    printf("Name %s <- after change\n", fullName);
    free(fullName);

    return 0;
}