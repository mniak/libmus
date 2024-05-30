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
	deg7 := d.singleOctave()
	return deg7 == 1 || deg7 == 4 || deg7 == 5
}

func (d Degree) singleOctave() Degree {
	return rmod(d, 1, 7)
}

func (d Degree) singleOctaveSemitones() int {
	deg7 := d.singleOctave()
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

func (d Degree) Octaves() int {
	return int(d-1) / 7
}

func (d Degree) semitones() int {
	s := d.singleOctaveSemitones()
	os := d.Octaves()
	result := s + (os * 12)
	return result
}

type Interval struct {
	Degree  Degree
	Quality Quality
}

func (i Interval) Semitones() int {
	s := i.Degree.semitones()
	off := i.Quality.Offset(i.Degree)
	return s + off
}
