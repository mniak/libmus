#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum PitchStep {
  C,
  D,
  E,
  F,
  G,
  A,
  B,
} PitchStep;

uintptr_t add(uintptr_t left, uintptr_t right);

uintptr_t add9(uintptr_t left, uintptr_t right);

uint8_t PitchStep_ToNumber(enum PitchStep step);

const char *PitchStep_ToString(enum PitchStep step);
