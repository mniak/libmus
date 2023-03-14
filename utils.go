package libmus

import "math/rand"

func truncateRange(value, min, max int) int {
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
