#include "Pitch.h"
#include "Utils.h"
#include <random>

namespace libmus {
	Pitch::Pitch()
	{
		this->octave = 4;
	}

	Pitch Pitch::Parse(string pitch)
	{
		return Pitch();
	}

	uniform_int_distribution<> octaveDistribution(Pitch::MIN_OCTAVE, Pitch::MAX_OCTAVE);
	Pitch Pitch::Random()
	{
		Pitch pitch;
		pitch.pitchClass = PitchClass::Random();
		pitch.octave = utils::generateRandom(octaveDistribution);
		return pitch;
	}

	Pitch Pitch::ExtendedRandom()
	{
		Pitch pitch;
		pitch.pitchClass = PitchClass::ExtendedRandom();
		pitch.octave = utils::generateRandom(octaveDistribution);
		return pitch;
	}

	int Pitch::GetStep()
	{
		return this->pitchClass.Step;
	}

	void Pitch::SetStep(int value)
	{
		this->pitchClass.Step = value;
	}

	int Pitch::GetAlteration()
	{
		return this->pitchClass.Alteration;
	}

	void Pitch::SetAlteration(int value)
	{
		this->pitchClass.Alteration = value;
	}

	int Pitch::GetOctave()
	{
		return this->octave;
	}

	void Pitch::SetOctave(int value)
	{
		this->octave = utils::truncateRange(value, MIN_OCTAVE, MAX_OCTAVE);
	}

	string Pitch::Name()
	{
		return string();
	}

	string Pitch::PrettyName()
	{
		return string();
	}

	string Pitch::FullName()
	{
		return string();
	}

}