#pragma once

#include <random>

using namespace std;

namespace libmus {
	namespace utils {
		int truncateRange(int value, int min, int max);

		int generateRandom(uniform_int_distribution<> distribution);
	}
}