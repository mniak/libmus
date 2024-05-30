package libmus

func PitchClassCFlat() PitchClass {
	return PitchClass{
		step:       StepC,
		alteration: AlterationFlat,
	}
}

func PitchClassC() PitchClass {
	return PitchClass{
		step: StepC,
	}
}

func PitchClassCSharp() PitchClass {
	return PitchClass{
		step:       StepC,
		alteration: AlterationSharp,
	}
}

func PitchClassDFlat() PitchClass {
	return PitchClass{
		step:       StepD,
		alteration: AlterationFlat,
	}
}

func PitchClassD() PitchClass {
	return PitchClass{
		step: StepD,
	}
}

func PitchClassDSharp() PitchClass {
	return PitchClass{
		step:       StepD,
		alteration: AlterationSharp,
	}
}

func PitchClassEFlat() PitchClass {
	return PitchClass{
		step:       StepE,
		alteration: AlterationFlat,
	}
}

func PitchClassE() PitchClass {
	return PitchClass{
		step: StepE,
	}
}

func PitchClassESharp() PitchClass {
	return PitchClass{
		step:       StepE,
		alteration: AlterationSharp,
	}
}

func PitchClassFFlat() PitchClass {
	return PitchClass{
		step:       StepF,
		alteration: AlterationFlat,
	}
}

func PitchClassF() PitchClass {
	return PitchClass{
		step: StepF,
	}
}

func PitchClassFSharp() PitchClass {
	return PitchClass{
		step:       StepF,
		alteration: AlterationSharp,
	}
}

func PitchClassGFlat() PitchClass {
	return PitchClass{
		step:       StepG,
		alteration: AlterationFlat,
	}
}

func PitchClassG() PitchClass {
	return PitchClass{
		step: StepG,
	}
}

func PitchClassGSharp() PitchClass {
	return PitchClass{
		step:       StepG,
		alteration: AlterationSharp,
	}
}

func PitchClassAFlat() PitchClass {
	return PitchClass{
		step:       StepA,
		alteration: AlterationFlat,
	}
}

func PitchClassA() PitchClass {
	return PitchClass{
		step: StepA,
	}
}

func PitchClassASharp() PitchClass {
	return PitchClass{
		step:       StepA,
		alteration: AlterationSharp,
	}
}

func PitchClassBFlat() PitchClass {
	return PitchClass{
		step:       StepB,
		alteration: AlterationFlat,
	}
}

func PitchClassB() PitchClass {
	return PitchClass{
		step: StepB,
	}
}

func PitchClassBSharp() PitchClass {
	return PitchClass{
		step:       StepB,
		alteration: AlterationSharp,
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
	pc.SetStep(pickRandom(steps))
	pc.SetAlteration(pickRandom(usualAlterations))
	return pc
}

func ExtendedRandomPitchClass() PitchClass {
	var pc PitchClass
	pc.SetStep(pickRandom(steps))
	pc.SetAlteration(pickRandom(alterations))
	return pc
}
