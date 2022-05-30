#include "pch.h"
#include "Pitch.h"

namespace libmus {
	Pitch::Pitch()
	{
		this->octave = 4;
	}

	Pitch Pitch::Parse(std::string pitch)
	{
		return Pitch();
	}

	Pitch Pitch::Random()
	{
		return Pitch();
	}

	Pitch Pitch::ExtendedRandom()
	{
		return Pitch();
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
		this->octave = value;
	}

	std::string Pitch::Name()
	{
		return std::string();
	}

	std::string Pitch::PrettyName()
	{
		return std::string();
	}

	std::string Pitch::FullName()
	{
		return std::string();
	}

}