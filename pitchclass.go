package libmus

import (
	"fmt"
	"regexp"
)

type PitchClass struct {
	step       Step
	alteration Alteration
}

func (pc PitchClass) GetStep() Step {
	return pc.step
}

func (pc *PitchClass) SetStep(value Step) {
	if value < StepC {
		return
	}

	pc.step = (value-StepC)%(StepB-StepC+1) + StepC
}

func (pc PitchClass) GetAlteration() Alteration {
	return pc.alteration
}

func (pc *PitchClass) SetAlteration(value Alteration) {
	pc.alteration = Alteration(trunc(value, AlterationDoubleFlat, AlterationDoubleSharp))
}

var regexParsePitchClass = regexp.MustCompile(fmt.Sprintf(
	`(?i)^ ?(double flat|bb|%c|flat|b|%c|natural|%c|sharp|#|%c|double sharp|##|%c)$`,
	DOUBLE_FLAT_SYMBOL, FLAT_SYMBOL, NATURAL_SYMBOL, SHARP_SYMBOL, DOUBLE_SHARP_SYMBOL,
))

func ParsePitchClass(text string) (PitchClass, error) {
	var result PitchClass
	var err error
	if len(text) == 0 {
		return result, ErrInvalidPitchStep
	}
	result.step, err = ParseStep(rune(text[0]))
	if err != nil {
		return result, err
	}
	result.alteration, err = ParseAlteration(text[1:])
	return result, err
}

func (pc PitchClass) String() string {
	return pc.Name()
}

func (pc PitchClass) Name() string {
	result := pc.step.Name()
	for i := AlterationSharp; i <= pc.alteration; i++ {
		result += "#"
	}
	for i := AlterationFlat; i >= pc.alteration; i-- {
		result += "b"
	}
	return result
}

func (pc PitchClass) PrettyName() string {
	result := pc.step.Name()
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

func (pc PitchClass) FullName() string {
	name := pc.step.Name()
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

func (pc PitchClass) OnOctave(octave int) Pitch {
	return Pitch{
		PitchClass: pc,
		Octave:     octave,
	}
}

func (pc PitchClass) number() int {
	var num int
	switch pc.step {
	default:
		fallthrough
	case StepC:
		num = 0
	case StepD:
		num = 2
	case StepE:
		num = 4
	case StepF:
		num = 5
	case StepG:
		num = 7
	case StepA:
		num = 9
	case StepB:
		num = 11
	}
	result := num + int(pc.alteration)
	return result
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

func (pc PitchClass) Previous() PitchClass {
	if pc.alteration > 0 {
		pc.alteration--
		return pc
	}
	if pc.step == StepC || pc.step == StepF {
		pc.step = pc.step.Previous()
		return pc
	}
	if pc.alteration == 0 {
		pc.alteration--
		return pc
	}
	if pc.alteration < 0 {
		pc.step = pc.step.Previous()
		pc.alteration++
	}
	return pc
}

func (pc PitchClass) normalized() PitchClass {
	pc.step = pc.step.normalized()
	pc.alteration = pc.alteration.normalized()
	return pc
}
