#pragma once

#include <string>

namespace libmus {

	class PitchClass
	{
	private:
		int step;
		int alteration;

	public:

		static const int MIN_STEP = 1;
		static const int MAX_STEP = 7;
		static const int MIN_ALTERATION = -2;
		static const int MAX_ALTERATION = 2;

		PitchClass();
		static PitchClass Random();
		static PitchClass ExtendedRandom();
		static PitchClass Parse(std::string text);

		std::string Name();
		std::string PrettyName();
		std::string FullName();

		__declspec(property(get = GetStep, put = SetStep)) int Step;
		int GetStep();
		void SetStep(int value);

		__declspec(property(get = GetAlteration, put = SetAlteration)) int Alteration;
		int GetAlteration();
		void SetAlteration(int value);
	};
}