#include "PitchClass.h"
#include "Utils.h"
#include <random>
#include <iostream>

using namespace std;

namespace libmus {

PitchClass::PitchClass() {
    this->step = 1;
    this->alteration = 0;
}

int PitchClass::GetStep() {
    return this->step;
}

void PitchClass::SetStep(int value) {
    if (value < MIN_STEP)
        return;

    this->step = (value - MIN_STEP) % (MAX_STEP - MIN_STEP + 1) + MIN_STEP;
}

int PitchClass::GetAlteration() {
    return this->alteration;
}

void PitchClass::SetAlteration(int value) {
    this->alteration = utils::truncateRange(value, MIN_ALTERATION, MAX_ALTERATION);
}

uniform_int_distribution<> usualAlterationDistribution(-1, 1);
uniform_int_distribution<> alterationDistribution(PitchClass::MIN_ALTERATION, PitchClass::MAX_ALTERATION);
uniform_int_distribution<> stepDistribution(PitchClass::MIN_STEP, PitchClass::MAX_STEP);

PitchClass PitchClass::Random() {
    PitchClass pc;
    pc.SetStep(utils::generateRandom(stepDistribution));
    pc.SetAlteration(utils::generateRandom(usualAlterationDistribution));
    return pc;
}

PitchClass PitchClass::ExtendedRandom() {
    PitchClass pc;
    pc.SetStep(utils::generateRandom(stepDistribution));
    pc.SetAlteration(utils::generateRandom(alterationDistribution));
    return pc;
}

const vector<u32string> NAMES = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};

const auto FLAT_SYMBOL = U'\x266D';
const auto SHARP_SYMBOL = U'\x266F';
const auto DOUBLE_FLAT_SYMBOL = U'\xD834DD2B';
const auto DOUBLE_SHARP_SYMBOL = U'\xD834DD2A';

PitchClass PitchClass::Parse(u32string value) {
    PitchClass newpc;
    auto head = value.substr(0, 1);
    auto tail = value.substr(1);

    for (auto iname = 0; iname < NAMES.size(); iname++) {
        auto name = NAMES[iname];

        if (name == head) {
            newpc.step = iname + 1;
            break;
        }
    }

    for (wchar_t ch : tail) {
        switch (ch) {
            case U'b':
            case FLAT_SYMBOL:
                newpc.alteration = newpc.alteration - 1;
                break;
            case U'#':
            case SHARP_SYMBOL:
                newpc.alteration = newpc.alteration + 1;
                break;
            case DOUBLE_FLAT_SYMBOL:
                newpc.alteration = newpc.alteration - 2;
                break;
            case DOUBLE_SHARP_SYMBOL:
                newpc.alteration = newpc.alteration + 2;
                break;
            default:
                return newpc;
        }
    }
}

u32string PitchClass::Name() {
    auto result = NAMES[this->step - 1];
    for (auto i = 1; i <= this->alteration; i++) {
        result = result + U'#';
    }
    for (auto i = -1; i >= this->alteration; i--) {
        result = result + U'b';
    }
    return result;
}

u32string PitchClass::PrettyName() {
    auto result = NAMES[this->step - 1];
    switch (this->alteration) {
        case -2:
            return result + DOUBLE_FLAT_SYMBOL;
        case -1:
            return result + FLAT_SYMBOL;
        case 1:
            return result + SHARP_SYMBOL;
        case 2:
            return result + DOUBLE_SHARP_SYMBOL;
    }
    return result;
}

u32string PitchClass::FullName() {
    auto name = NAMES[this->step - 1];
    switch (this->alteration) {
        case -2:
            return name + U" double flat";
        case -1:
            return name + U" flat";
        case 1:
            return name + U" sharp";
        case 2:
            return name + U" double sharp";
        default:
            return name;
    }
}
}