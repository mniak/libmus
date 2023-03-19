#include "../libmus.h"
#include <stdio.h>

int main() {
    printf("-- begin call --\n");
    MyFunction(1,2,"test");
    printf("-- end call --\n");
    return 0;
}