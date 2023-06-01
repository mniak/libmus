package libmus

import (
	"fmt"
	"regexp"
	"strings"
)

type Step int

const (
	StepC Step = 1
	StepD Step = 2
	StepE Step = 3
	StepF Step = 4
	StepG Step = 5
	StepA Step = 6
	StepB Step = 7
)

type Alteration int

const (
	AlterationDoubleFlat  Alteration = -2
	AlterationFlat        Alteration = -1
	AlterationNatural     Alteration = 0
	AlterationSharp       Alteration = 1
	AlterationDoubleSharp Alteration = 2
)

const (
	MIN_STEP             = StepC
	MAX_STEP             = StepB
	MIN_USUAL_ALTERATION = AlterationFlat
	MAX_USUAL_ALTERATION = AlterationSharp
	MIN_ALTERATION       = AlterationDoubleFlat
	MAX_ALTERATION       = AlterationDoubleSharp
)

type PitchClass struct {
	step       Step
	alteration Alteration
}

func PitchClassC() PitchClass {
	return PitchClass{
		step: StepC,
	}
}

func NewPitchClass() PitchClass {
	return PitchClass{
		step:       StepC,
		alteration: AlterationNatural,
	}
}

func (pc *PitchClass) GetStep() Step {
	return pc.step
}

func (pc *PitchClass) SetStep(value Step) {
	if value < MIN_STEP {
		return
	}

	pc.step = (value-MIN_STEP)%(MAX_STEP-MIN_STEP+1) + MIN_STEP
}

func (pc *PitchClass) GetAlteration() Alteration {
	return pc.alteration
}

func (pc *PitchClass) SetAlteration(value Alteration) {
	pc.alteration = Alteration(truncateRange(int(value), int(MIN_ALTERATION), int(MAX_ALTERATION)))
}

var (
	usualAlterations []Alteration
	alterations      []Alteration
	steps            []Step
)

func init() {
	for i := MIN_USUAL_ALTERATION; i <= MAX_USUAL_ALTERATION; i++ {
		usualAlterations = append(usualAlterations, i)
	}

	for i := MIN_ALTERATION; i <= MAX_ALTERATION; i++ {
		alterations = append(alterations, i)
	}

	for i := MIN_STEP; i <= MAX_STEP; i++ {
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

var regexParsePitchClass = regexp.MustCompile(fmt.Sprintf(
	`(?i)^ ?(double flat|bb|%c|flat|b|%c|natural|%c|sharp|#|%c|double sharp|##|%c)$`,
	DOUBLE_FLAT_SYMBOL, FLAT_SYMBOL, NATURAL_SYMBOL, SHARP_SYMBOL, DOUBLE_SHARP_SYMBOL,
))

func ParsePitchClass(text string) PitchClass {
	var newpc PitchClass
	head := text[0:1]
	tail := text[1:]

	for iname, name := range NAMES {
		if name == head {
			newpc.step = Step(iname + 1)
			break
		}
	}

	isMatch := regexParsePitchClass.MatchString(tail)
	if !isMatch {
		return newpc
	}
	match := strings.ToLower(regexParsePitchClass.FindStringSubmatch(tail)[1])

	switch match {
	case "flat", string(FLAT_SYMBOL), string('b'):
		newpc.alteration = newpc.alteration - 1

	case "sharp", string(SHARP_SYMBOL), string('#'):
		newpc.alteration = newpc.alteration + 1

	case "double flat", string(DOUBLE_FLAT_SYMBOL), "bb":
		newpc.alteration = newpc.alteration - 2

	case "double sharp", string(DOUBLE_SHARP_SYMBOL), "##":
		newpc.alteration = newpc.alteration + 2

	default:
		fallthrough
	case "", "natural", string(NATURAL_SYMBOL):
		return newpc
	}
	return newpc
}

func (pc *PitchClass) Name() string {
	result := NAMES[pc.step-1]
	for i := AlterationSharp; i <= pc.alteration; i++ {
		result = result + "#"
	}
	for i := AlterationFlat; i >= pc.alteration; i-- {
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
