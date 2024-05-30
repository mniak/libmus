package libmus

type cMajorScale struct{}

func CMajorScale() cMajorScale {
	return cMajorScale{}
}

func (s cMajorScale) Transpose(p Pitch, i Interval) Pitch {
	step := p.PitchClass.GetStep()
	p.PitchClass.SetStep(step + Step(i.Number-1))
	// p.octave += (i.Number - 1) % 7
	return p
}

type Interval struct {
	Number int
	// Quality string
	// Descending bool
}
