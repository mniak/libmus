package libmus

type Quality string

const (
	Augmented  Quality = "augmented"
	Major      Quality = "major"
	Perfect    Quality = "perfect"
	Minor      Quality = "minor"
	Diminished Quality = "diminished"
)

func (q Quality) Name() string {
	switch q {
	case Augmented:
		return "augmented"
	case Major:
		return "major"
	case Perfect:
		return "perfect"
	case Minor:
		return "minor"
	case Diminished:
		return "diminished"
	default:
		return "invalid quality"
	}
}

func (q Quality) String() string {
	return q.Name()
}

func (q Quality) Offset(degree Degree) int {
	switch q {
	default:
		fallthrough
	case Major, Perfect:
		return 0
	case Minor:
		return -1
	case Diminished:
		if degree.Perfectable() {
			return -1
		} else {
			return -2
		}
	case Augmented:
		if degree.Perfectable() {
			return +1
		} else {
			return +2
		}
	}
}

type Degree int

func (d Degree) Perfectable() bool {
	deg7 := d.asSingleOctave()
	return deg7 == 1 || deg7 == 4 || deg7 == 5
}

func (d Degree) asSingleOctave() Degree {
	if d == 0 {
		return 1
	} else if d > 0 {
		return rmod(d, 1, 7)
	} else {
		return rmod(d, 7, -1)
	}
}

func (d Degree) singleOctaveSemitones() int {
	deg7 := d.asSingleOctave()
	switch deg7 {
	default:
		fallthrough
	case 1:
		return 0
	case 2:
		return 2
	case 3:
		return 4
	case 4:
		return 5
	case 5:
		return 7
	case 6:
		return 9
	case 7:
		return 11
	}
}

func (d Degree) octaveSpan() int {
	return int(d-1) / 7
}

func (d Degree) semitones() int {
	s := d.singleOctaveSemitones()
	os := d.octaveSpan()
	result := s + (os * 12)
	return result
}

type Direction bool

const (
	Ascending  Direction = false
	Descending Direction = true
)

type Interval struct {
	Degree    Degree
	Quality   Quality
	Direction Direction
}

func (i Interval) Semitones() int {
	offset := i.Quality.Offset(i.Degree)
	s := i.Degree.semitones()

	result := s + offset

	if i.Direction == Descending {
		return -result
	}
	return result
}

func (i Interval) Ascending() Interval {
	i.Direction = Ascending
	return i
}

func (i Interval) Descending() Interval {
	i.Direction = Descending
	return i
}

func (i Interval) Reverse() Interval {
	i.Direction = !i.Direction
	return i
}
