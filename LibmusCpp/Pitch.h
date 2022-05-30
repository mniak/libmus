#pragma once
#include <string>
#include "PitchClass.h"

namespace libmus {
	class Pitch
	{
	public:
		Pitch();
		static Pitch Parse(std::string pitch);
		static Pitch Random();
		static Pitch ExtendedRandom();

		int Step;
		int Alteration;
		int Octave;

		std::string Name();
		std::string PrettyName();
		std::string FullName();
	};
}