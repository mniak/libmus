#pragma once

#include <random>

namespace libmus::constants {
const std::vector<std::u32string> NAMES = {U"C", U"D", U"E", U"F", U"G", U"A", U"B"};

const auto FLAT_SYMBOL = U'\x266D';
const auto SHARP_SYMBOL = U'\x266F';
const auto DOUBLE_FLAT_SYMBOL = U'\x1D12B';
const auto DOUBLE_SHARP_SYMBOL = U'\x1D12A';

const std::vector<std::u32string> SUPERSCRIPT_OCTAVES = {U"\x2070", U"\x00B9", U"\x00B2",      U"\x00B3",
                                                         U"\x2074", U"\x2075", U"\x2076",      U"\x2077",
                                                         U"\x2078", U"\x2079", U"\x00B9\x2070"};

}