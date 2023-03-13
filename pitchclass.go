package libmus

const (
	MIN_STEP       = 1
	MAX_STEP       = 7
	MIN_ALTERATION = -2
	MAX_ALTERATION = 2
)

type PitchClass struct {
	step       int
	alteration int
}

func NewPitchClass() PitchClass {
	return PitchClass{
		step:       1,
		alteration: 0,
	}
}

func (pc *PitchClass) GetStep() int {
	return pc.step
}

func (pc *PitchClass) SetStep(value int) {
	if value < MIN_STEP {
		return
	}

	pc.step = (value-MIN_STEP)%(MAX_STEP-MIN_STEP+1) + MIN_STEP
}

func (pc *PitchClass) GetAlteration() int {
	return pc.alteration
}

func (pc *PitchClass) SetAlteration(value int) {
	pc.alteration = truncateRange(value, MIN_ALTERATION, MAX_ALTERATION)
}

var (
	usualAlterations []int
	alterations      []int
	steps            []int
)

func init() {
	for i := -1; i <= 1; i++ {
		usualAlterations = append(usualAlterations, i)
	}

	for i := MIN_ALTERATION; MAX_ALTERATION <= 1; i++ {
		alterations = append(alterations, i)
	}

	for i := MIN_STEP; MAX_STEP <= 1; i++ {
		steps = append(steps, i)
	}
}

func RandomPitchClass() PitchClass {
	var pc PitchClass
	pc.SetStep(generateRandom(steps))
	pc.SetAlteration(generateRandom(usualAlterations))
	return pc
}

func ExtendedRandomPitchClass() PitchClass {
	var pc PitchClass
	pc.SetStep(generateRandom(steps))
	pc.SetAlteration(generateRandom(alterations))
	return pc
}

func ParsePitchClass(text string) PitchClass {
	var newpc PitchClass
	head := text[0:1]
	tail := text[1:]

	for iname, name := range NAMES {
		if name == head {
			newpc.step = iname + 1
			break
		}
	}

	for _, ch := range tail {
		switch ch {
		case 'b':
		case FLAT_SYMBOL:
			newpc.alteration = newpc.alteration - 1
		case '#':
		case SHARP_SYMBOL:
			newpc.alteration = newpc.alteration + 1
		case DOUBLE_FLAT_SYMBOL:
			newpc.alteration = newpc.alteration - 2
		case DOUBLE_SHARP_SYMBOL:
			newpc.alteration = newpc.alteration + 2
		default:
			return newpc
		}
	}
	return newpc
}

func (pc *PitchClass) Name() string {
	result := NAMES[pc.step-1]
	for i := 1; i <= pc.alteration; i++ {
		result = result + "#"
	}
	for i := -1; i >= pc.alteration; i-- {
		result = result + "b"
	}
	return result
}

func (pc *PitchClass) PrettyName() string {
	result := NAMES[pc.step-1]
	switch pc.alteration {
	case -2:
		return result + string(DOUBLE_FLAT_SYMBOL)
	case -1:
		return result + string(FLAT_SYMBOL)
	case 1:
		return result + string(SHARP_SYMBOL)
	case 2:
		return result + string(DOUBLE_SHARP_SYMBOL)
	}
	return result
}

func (pc *PitchClass) FullName() string {
	name := NAMES[pc.step-1]
	switch pc.alteration {
	case -2:
		return name + " double flat"
	case -1:
		return name + " flat"
	case 1:
		return name + " sharp"
	case 2:
		return name + " double sharp"
	default:
		return name
	}
}
