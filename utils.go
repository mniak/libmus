package libmus

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
	var t T
	return t
}
