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
    static Pitch Parse(std::wstring pitch);
    static Pitch Random();
    static Pitch ExtendedRandom();

    int GetStep();
    void SetStep(int value);

    int GetAlteration();
    void SetAlteration(int value);

    int GetOctave();
    void SetOctave(int value);

    std::wstring Name();
    std::wstring PrettyName();
    std::wstring FullName();

    static const int MIN_OCTAVE = 0;
    static const int MAX_OCTAVE = 10;
};
}