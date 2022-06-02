#include "pch.h"
#include "Utils.h"
#include <cmath>
#include <random>

int libmus::utils::truncateRange(int value, int min, int max) {
    value = (value >= max) * max + (value < max) * value;
    value = (value <= min) * min + (value > min) * value;
    return value;
}

std::random_device randDevice;
std::mt19937 generator(randDevice());
std::uniform_int_distribution<> usualAlterationDistribution(-1, 1);

int libmus::utils::generateRandom(std::uniform_int_distribution<> distribution) {
    return distribution(generator);
}
