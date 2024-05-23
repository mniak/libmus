package libmus

import (
	"fmt"
	"regexp"
	"strings"
)

type Alteration int

const (
	AlterationDoubleFlat  Alteration = -2
	AlterationFlat        Alteration = -1
	AlterationNatural     Alteration = 0
	AlterationSharp       Alteration = 1
	AlterationDoubleSharp Alteration = 2
)

type PitchClass struct {
	step       Step
	alteration Alteration
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
	if value < StepC {
		return
	}

	pc.step = (value-StepC)%(StepB-StepC+1) + StepC
}

func (pc *PitchClass) GetAlteration() Alteration {
	return pc.alteration
}

func (pc *PitchClass) SetAlteration(value Alteration) {
	pc.alteration = Alteration(truncateRange(value, AlterationDoubleFlat, AlterationDoubleSharp))
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

func (pc PitchClass) String() string {
	return pc.Name()
}

func (pc *PitchClass) Name() string {
	result := NAMES[pc.step]
	for i := AlterationSharp; i <= pc.alteration; i++ {
		result = result + "#"
	}
	for i := AlterationFlat; i >= pc.alteration; i-- {
		result = result + "b"
	}
	return result
}

func (pc *PitchClass) PrettyName() string {
	result := NAMES[pc.step]
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
	name := NAMES[pc.step]
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

func (pc *PitchClass) OnOctave(octave int) Pitch {
	return Pitch{
		pitchClass: *pc,
		octave:     octave,
	}
}

func (pc *PitchClass) Number() int {
	switch pc.step {
	default:
		fallthrough
	case StepC:
		return 0
	case StepD:
		return 2
	case StepE:
		return 4
	case StepF:
		return 5
	case StepG:
		return 7
	case StepA:
		return 9
	case StepB:
		return 11
	}
}

func (pc PitchClass) Next() PitchClass {
	if pc.alteration < 0 {
		pc.alteration++
		return pc
	}
	if pc.step == StepB || pc.step == StepE {
		pc.step = pc.step.Next()
		return pc
	}
	if pc.alteration == 0 {
		pc.alteration++
		return pc
	}
	if pc.alteration > 0 {
		pc.step = pc.step.Next()
		pc.alteration--
	}
	return pc
}
