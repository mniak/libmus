#pragma once
#include <string>
#include "PitchClass.h"

namespace libmus {
class Pitch {
   private:
    PitchClass pitchClass;
    int octave;

   public:
    Pitch();
    static Pitch Parse(std::u32string text);
    static Pitch Random();
    static Pitch ExtendedRandom();

    int GetStep();
    void SetStep(int value);

    int GetAlteration();
    void SetAlteration(int value);

    int GetOctave();
    void SetOctave(int value);

    std::u32string Name();
    std::u32string PrettyName();
    std::u32string FullName();

    static const int MIN_OCTAVE = 0;
    static const int MAX_OCTAVE = 10;
};
}