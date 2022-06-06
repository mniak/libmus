#pragma once

#include <random>

namespace libmus::utils {
int truncateRange(int value, int min, int max);

int generateRandom(std::uniform_int_distribution<> distribution);
}