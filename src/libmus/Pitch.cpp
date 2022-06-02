#include "Pitch.h"
#include "Utils.h"
#include <random>

namespace libmus {
Pitch::Pitch() {
    this->octave = 4;
}

Pitch Pitch::Parse(wstring pitch) {
    return Pitch();
}

uniform_int_distribution<> octaveDistribution(Pitch::MIN_OCTAVE, Pitch::MAX_OCTAVE);
Pitch Pitch::Random() {
    Pitch pitch;
    pitch.pitchClass = PitchClass::Random();
    pitch.octave = utils::generateRandom(octaveDistribution);
    return pitch;
}

Pitch Pitch::ExtendedRandom() {
    Pitch pitch;
    pitch.pitchClass = PitchClass::ExtendedRandom();
    pitch.octave = utils::generateRandom(octaveDistribution);
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
    this->octave = utils::truncateRange(value, MIN_OCTAVE, MAX_OCTAVE);
}

wstring Pitch::Name() {
    return this->pitchClass.Name();
}

wstring Pitch::PrettyName() {
    return this->pitchClass.Name();
}

wstring Pitch::FullName() {
    return this->pitchClass.Name();
}

}