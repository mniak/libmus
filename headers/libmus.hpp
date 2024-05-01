#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class PitchStep {
  C,
  D,
  E,
  F,
  G,
  A,
  B,
};

extern "C" {

uintptr_t add(uintptr_t left, uintptr_t right);

uintptr_t add9(uintptr_t left, uintptr_t right);

uint8_t PitchStep_ToNumber(PitchStep step);

const char *PitchStep_ToString(PitchStep step);

} // extern "C"
