#include "libmus.h"
#include "utils.h"
#include "constants.h"
#include <random>
#include <locale>
#include <codecvt>

namespace libmus {

using namespace std;
using namespace utils;
using namespace constants;

Pitch::Pitch() {
    this->octave = 4;
}

Pitch Pitch::Parse(u32string text) {
    Pitch pitch;
    auto head = text[0];
    auto tail = text.substr(1);

    wstring_convert<codecvt_utf8<char32_t>, char32_t> convert;

    for (auto iname = 0; iname < NAMES.size(); iname++) {
        auto name = NAMES[iname][0];

        if (name == head) {
            pitch.pitchClass.SetStep(iname + 1);
            break;
        }
    }
    for (char32_t ch : tail) {
        try {
            pitch.pitchClass.SetStep(stoi(convert.to_bytes(tail)));
        } catch (invalid_argument& err) {
        }

        head = tail[0];
        tail = tail.substr(1);

        if (head == U'b' || head == FLAT_SYMBOL) {
            pitch.pitchClass.SetAlteration(pitch.GetAlteration() - 1);
        } else if (head == U'#' || head == SHARP_SYMBOL) {
            pitch.pitchClass.SetAlteration(pitch.GetAlteration() + 1);
        } else {
            return pitch;
        }
    }
    return pitch;
}

uniform_int_distribution<> octaveDistribution(Pitch::MIN_OCTAVE, Pitch::MAX_OCTAVE);
Pitch Pitch::Random() {
    Pitch pitch;
    pitch.pitchClass = PitchClass::Random();
    pitch.octave = generateRandom(octaveDistribution);
    return pitch;
}

Pitch Pitch::ExtendedRandom() {
    Pitch pitch;
    pitch.pitchClass = PitchClass::ExtendedRandom();
    pitch.octave = generateRandom(octaveDistribution);
    return pitch;
}

int Pitch::GetStep() {
    return this->pitchClass.GetStep();
}

void Pitch::SetStep(int value) {
    this->pitchClass.SetStep(value);
}

int Pitch::GetAlteration() {
    return this->pitchClass.GetAlteration();
}

void Pitch::SetAlteration(int value) {
    this->pitchClass.SetAlteration(value);
}

int Pitch::GetOctave() {
    return this->octave;
}

void Pitch::SetOctave(int value) {
    this->octave = truncateRange(value, MIN_OCTAVE, MAX_OCTAVE);
}

u32string Pitch::Name() {
    auto octaveString = to_string(this->octave);
    u32string octave32(octaveString.begin(), octaveString.end());
    return this->pitchClass.Name() + octave32;
}

u32string Pitch::PrettyName() {
    auto oct = SUPERSCRIPT_OCTAVES[this->octave];
    return this->pitchClass.PrettyName() + oct;
}

u32string Pitch::FullName() {
    auto octaveString = to_string(this->octave);
    u32string octave32(octaveString.begin(), octaveString.end());
    return this->pitchClass.FullName() + U" " + octave32;
}
}