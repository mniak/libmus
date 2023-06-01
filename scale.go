package libmus

type cMajorScale struct{}

func CMajorScale() cMajorScale {
	return cMajorScale{}
}

func (s cMajorScale) Transpose(p Pitch, i Interval) Pitch {
	step := p.pitchClass.GetStep()
	p.pitchClass.SetStep(step + Step(i.Number-1))
	// p.octave += (i.Number - 1) % 7
	return p
}

type Interval struct {
	Number int
	// Quality string
	// Descending bool
}
