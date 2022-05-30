#include "pch.h"
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
		pc.Step = utils::generateRandom(stepDistribution);
		pc.Alteration = utils::generateRandom(usualAlterationDistribution);
		return pc;
	}

	PitchClass PitchClass::ExtendedRandom() {
		PitchClass pc;
		pc.Step = utils::generateRandom(stepDistribution);
		pc.Alteration = utils::generateRandom(alterationDistribution);
		return pc;
	}
	PitchClass PitchClass::Parse(std::string text)
	{
		return PitchClass();
	}

	const vector<string> NAMES = { "C", "D", "E", "F", "G", "A", "B" };
	std::string PitchClass::Name()
	{
		auto result = NAMES[this->Step -1];
		for (auto i = 1; i <= this->alteration; i++) {
			result = result + "#";
		}
		for (auto i = -1; i >= this->alteration; i--) {
			result = result + "b";
		}
		return result;
	}

	std::string PitchClass::PrettyName()
	{
		return std::string();
	}
	std::string PitchClass::FullName()
	{
		return std::string();
	}
}