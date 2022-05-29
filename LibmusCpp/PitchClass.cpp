#include "pch.h"
#include "PitchClass.h"
#include "Utils.h"

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