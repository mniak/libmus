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

uint8_t PitchStep_ToNumber(enum PitchStep step);

const char *PitchStep_ToString(enum PitchStep step);
