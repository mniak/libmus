#include "pch.h"

#include <cmath>
#include "Utils.h"

int truncateRange(int value, int min, int max) {
	value = (value >= max) * max + (value < max) * value;
	value = (value <= min) * min + (value > min) * value;
	return value;
}