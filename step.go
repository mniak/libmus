package libmus

import "errors"

type Step int

var (
	STEP_NAMES          = []string{"C", "D", "E", "F", "G", "A", "B"}
	ErrInvalidPitchStep = errors.New("invalid pitch step")
)

const (
	StepC Step = 1
	StepD Step = 2
	StepE Step = 3
	StepF Step = 4
	StepG Step = 5
	StepA Step = 6
	StepB Step = 7
)

func ParseStep(r rune) (Step, error) {
	for iname, name := range STEP_NAMES {
		if name == string(r) {
			return Step(iname + 1), nil
		}
	}
	return StepC, ErrInvalidPitchStep
}

func (s Step) Next() Step {
	return (s + 1).normalized()
}

func (s Step) Previous() Step {
	return (s - 1).normalized()
}

func (s Step) normalized() Step {
	i := s - 1
	i %= 7
	if i < 0 {
		i += 7
	}
	return i + 1
}

func (s Step) Name() string {
	return STEP_NAMES[s.normalized()-1]
}

func (s Step) String() string {
	return s.Name()
}
