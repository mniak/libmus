package libmus

import "math/rand"

func trunc[T ~int](value, min, max T) T {
	if value >= max {
		return max
	}
	if value <= min {
		return min
	}
	return value
}

// mod: Modulo operation
// Returns the remainder of the division of `value` by `divisor`.
// The result will always be positive
func mod[T ~int](value, divisor T) T {
	value = value % divisor
	if value < 0 {
		value += divisor
	}
	return value
}

// rmod: Range modulo operation
// Keep the value in the range [min,max].
// The value "rotates" if greater than `max` or if smaller than `min`.
//
// Examples:
//
//	max + 1 =  min
//	max + 2 =  min + 1
//	max + 3 =  min + 2
//
//	min - 1 =  max
//	min - 2 =  max - 1
//	min - 3 =  max - 2
//
// rmod(n, 0, b) is equivalent to mod(n, b+1)
func rmod[T ~int](n, min, max T) T {
	n -= min
	n = mod(n, max-min+1)
	n += min
	return n
}

func pickRandom[T any](items []T) T {
	idx := rand.Intn(len(items))
	return items[idx]
}
