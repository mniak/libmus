package libmus

type Step int

const (
	StepC Step = 0
	StepD Step = 1
	StepE Step = 2
	StepF Step = 3
	StepG Step = 4
	StepA Step = 5
	StepB Step = 6
)

func (s Step) Next() Step {
	return (s + 1).normalized()
}

func (s Step) normalized() Step {
	return s % 7
}
