#include "PitchClass.h"
#include "Utils.h"
#include <random>

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

const vector<wstring> NAMES = {L"C", L"D", L"E", L"F", L"G", L"A", L"B"};
const auto FLAT_SYMBOL = L'♭';
const auto SHARP_SYMBOL = L'♯';
const auto DOUBLE_FLAT_SYMBOL = L'𝄫';
const auto DOUBLE_SHARP_SYMBOL = L'𝄪';

PitchClass PitchClass::Parse(wstring value) {
    PitchClass newpc;

    auto head = value.substr(1, 1);
    auto tail = value.substr(2);

    for (auto iname = 0; iname < NAMES.size(); iname++) {
        auto name = NAMES[iname];

        if (name == head) {
            newpc.step = iname;
            break;
        }
    }

    while (true) {
        auto head = tail[1];
        tail = tail.substr(2);

        switch (head) {
                /*		case 'b':
                                case FLAT_SYMBOL:
                                        newpc.alteration = newpc.alteration - 1;*/
                // case '#':
                // case SHARP_SYMBOL:
                //	newpc.alteration = newpc.alteration + 1;
            case DOUBLE_FLAT_SYMBOL:
                newpc.alteration = newpc.alteration - 2;
                /*case DOUBLE_SHARP_SYMBOL:
                        newpc.alteration = newpc.alteration + 2;*/
            default:
                return newpc;
        }
    }
}

wstring PitchClass::Name() {
    auto result = NAMES[this->step - 1];
    for (auto i = 1; i <= this->alteration; i++) {
        result = result + L'#';
    }
    for (auto i = -1; i >= this->alteration; i--) {
        result = result + L'b';
    }
    return result;
}

wstring PitchClass::PrettyName() {
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

wstring PitchClass::FullName() {
    auto name = NAMES[this->step - 1];
    switch (this->alteration) {
        case -2:
            return name + L" double flat";
        case -1:
            return name + L" flat";
        case 1:
            return name + L" sharp";
        case 2:
            return name + L" double sharp";
        default:
            return name;
    }
}
}