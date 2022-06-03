#pragma once

#include <random>

using namespace std;

namespace libmus::utils {
int truncateRange(int value, int min, int max);

int generateRandom(uniform_int_distribution<> distribution);
}