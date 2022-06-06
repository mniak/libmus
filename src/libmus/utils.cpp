#include "libmus.h"
#include "utils.h"
#include <cmath>
#include <random>

namespace libmus::utils {
int truncateRange(int value, int min, int max) {
    value = (value >= max) * max + (value < max) * value;
    value = (value <= min) * min + (value > min) * value;
    return value;
}

std::random_device randDevice;
std::mt19937 generator(randDevice());
std::uniform_int_distribution<> usualAlterationDistribution(-1, 1);

int generateRandom(std::uniform_int_distribution<> distribution) {
    return distribution(generator);
}
}
