package libmus

func Unison() Interval {
	return Interval{
		Degree:  1,
		Quality: Perfect,
	}
}

func MajorThird() Interval {
	return Interval{
		Degree:  3,
		Quality: Major,
	}
}

func Octave() Interval {
	return Interval{
		Degree:  8,
		Quality: Perfect,
	}
}
