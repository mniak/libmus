package libmus

func PitchClassC() PitchClass {
	return PitchClass{
		step: StepC,
	}
}

func PitchClassD() PitchClass {
	return PitchClass{
		step: StepD,
	}
}

func PitchClassE() PitchClass {
	return PitchClass{
		step: StepE,
	}
}

func PitchClassF() PitchClass {
	return PitchClass{
		step: StepF,
	}
}

func PitchClassG() PitchClass {
	return PitchClass{
		step: StepG,
	}
}

func PitchClassA() PitchClass {
	return PitchClass{
		step: StepA,
	}
}

func PitchClassB() PitchClass {
	return PitchClass{
		step: StepB,
	}
}

var (
	usualAlterations []Alteration
	alterations      []Alteration
	steps            []Step
)

func init() {
	for i := AlterationFlat; i <= AlterationSharp; i++ {
		usualAlterations = append(usualAlterations, i)
	}

	for i := AlterationDoubleFlat; i <= AlterationDoubleSharp; i++ {
		alterations = append(alterations, i)
	}

	for i := StepC; i <= StepB; i++ {
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
