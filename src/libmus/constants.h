#pragma once

#include <random>

using namespace std;

namespace libmus::constants {
const vector<u32string> NAMES = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};

const auto FLAT_SYMBOL = U'\x266D';
const auto SHARP_SYMBOL = U'\x266F';
const auto DOUBLE_FLAT_SYMBOL = U'\xD834DD2B';
const auto DOUBLE_SHARP_SYMBOL = U'\xD834DD2A';
}