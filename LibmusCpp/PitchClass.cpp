#include "pch.h"
#include "PitchClass.h"
#include "Utils.h"
#include <random>

#define PITCHCLASS_MIN_STEP 1
#define PITCHCLASS_MAX_STEP 7
#define PITCHCLASS_MIN_ALTERATION -2
#define PITCHCLASS_MAX_ALTERATION 2


PitchClass::PitchClass() {
	this->step = 1;
	this->alteration = 0;
}

int PitchClass::GetStep() {
	return this->step;
}

void PitchClass::SetStep(int value) {
	if (value < PITCHCLASS_MIN_STEP)
		return;

	this->step = (value - PITCHCLASS_MIN_STEP) % (PITCHCLASS_MAX_STEP - PITCHCLASS_MIN_STEP + 1) + PITCHCLASS_MIN_STEP;
}

int PitchClass::GetAlteration() {
	return this->alteration;
}

void PitchClass::SetAlteration(int value) {
	this->alteration = truncateRange(value, PITCHCLASS_MIN_ALTERATION, PITCHCLASS_MAX_ALTERATION);
}

std::random_device randDevice;
std::mt19937 generator(randDevice());
std::uniform_int_distribution<> usualAlterationDistribution(-1, 1);
std::uniform_int_distribution<> alterationDistribution(PITCHCLASS_MIN_ALTERATION, PITCHCLASS_MAX_ALTERATION);
std::uniform_int_distribution<> stepDistribution(PITCHCLASS_MIN_STEP, PITCHCLASS_MAX_STEP);

PitchClass PitchClass::Random() {
	PitchClass pc;
	pc.Step = stepDistribution(generator);
	pc.Alteration = usualAlterationDistribution(generator);
	return pc;
}

PitchClass PitchClass::ExtendedRandom() {
	PitchClass pc;
	pc.Step = stepDistribution(generator);
	pc.Alteration = alterationDistribution(generator);
	return pc;
}