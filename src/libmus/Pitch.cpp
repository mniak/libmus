#include "libmus.h"
#include "utils.h"
#include "constants.h"
#include <random>

namespace libmus {

using namespace utils;

Pitch::Pitch() {
    this->octave = 4;
}

Pitch Pitch::Parse(u32string pitch) {
    return Pitch();
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
    auto oct = constants::SUPERSCRIPT_OCTAVES[this->octave];
    return this->pitchClass.PrettyName() + oct;
}

u32string Pitch::FullName() {
    auto octaveString = to_string(this->octave);
    u32string octave32(octaveString.begin(), octaveString.end());
    return this->pitchClass.FullName() + U" " + octave32;
}

}