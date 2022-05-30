#include "pch.h"
#include "PitchClass.h"
#include "Utils.h"
#include <random>

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
		this->alteration = truncateRange(value, MIN_ALTERATION, MAX_ALTERATION);
	}

	std::random_device randDevice;
	std::mt19937 generator(randDevice());
	std::uniform_int_distribution<> usualAlterationDistribution(-1, 1);
	std::uniform_int_distribution<> alterationDistribution(PitchClass::MIN_ALTERATION, PitchClass::MAX_ALTERATION);
	std::uniform_int_distribution<> stepDistribution(PitchClass::MIN_STEP, PitchClass::MAX_STEP);

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
}