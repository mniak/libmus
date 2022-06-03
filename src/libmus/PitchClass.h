#pragma once

#include <string>

namespace libmus {

class PitchClass {
   private:
    int step;
    int alteration;

   public:
    static const int MIN_STEP = 1;
    static const int MAX_STEP = 7;
    static const int MIN_ALTERATION = -2;
    static const int MAX_ALTERATION = 2;

    PitchClass();
    static PitchClass Random();
    static PitchClass ExtendedRandom();
    static PitchClass Parse(std::u32string text);

    std::u32string Name();
    std::u32string PrettyName();
    std::u32string FullName();

    int GetStep();
    void SetStep(int value);

    int GetAlteration();
    void SetAlteration(int value);
};
}  // namespace libmus