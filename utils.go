package libmus

import "math/rand"

func truncateRange[T ~int](value, min, max T) T {
	if value >= max {
		return max
	}
	if value <= min {
		return min
	}
	return value
}

func generateRandom[T any](items []T) T {
	idx := rand.Intn(len(items))
	return items[idx]
}
