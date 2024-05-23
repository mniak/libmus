package libmus

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_PitchClass_Step_Constructor_should_set_initial_values(t *testing.T) {
	pc := NewPitchClass()

	assert.Equal(t, StepC, pc.GetStep())
	assert.Equal(t, AlterationNatural, pc.GetAlteration())
}

func Test_PitchClass_Step_Normal_values_should_work(t *testing.T) {
	var pc PitchClass

	pc.SetStep(StepC)
	assert.Equal(t, StepC, pc.GetStep())

	pc.SetStep(StepD)
	assert.Equal(t, StepD, pc.GetStep())

	pc.SetStep(StepE)
	assert.Equal(t, StepE, pc.GetStep())

	pc.SetStep(StepF)
	assert.Equal(t, StepF, pc.GetStep())

	pc.SetStep(StepG)
	assert.Equal(t, StepG, pc.GetStep())

	pc.SetStep(StepA)
	assert.Equal(t, StepA, pc.GetStep())

	pc.SetStep(StepB)
	assert.Equal(t, StepB, pc.GetStep())
}

func Test_PitchClass_Step_Bigger_values_should_be_normalized(t *testing.T) {
	var pc PitchClass

	pc.SetStep(8)
	assert.Equal(t, StepC, pc.GetStep())

	pc.SetStep(9)
	assert.Equal(t, StepD, pc.GetStep())

	pc.SetStep(10)
	assert.Equal(t, StepE, pc.GetStep())

	pc.SetStep(11)
	assert.Equal(t, StepF, pc.GetStep())

	pc.SetStep(12)
	assert.Equal(t, StepG, pc.GetStep())

	pc.SetStep(13)
	assert.Equal(t, StepA, pc.GetStep())

	pc.SetStep(14)
	assert.Equal(t, StepB, pc.GetStep())

	pc.SetStep(15)
	assert.Equal(t, StepC, pc.GetStep())
}

func Test_PitchClass_Step_Attributing_zero_or_negative_should_do_nothing(t *testing.T) {
	var pc PitchClass

	for goodValue := StepC; goodValue <= StepB; goodValue++ {
		for badValue := Step(-1); badValue >= Step(-20); badValue-- {
			pc.SetStep(goodValue)
			pc.SetStep(badValue)
			assert.Equal(t, goodValue, pc.GetStep())
		}
	}
}

func Test_PitchClass_Alteration_When_value_is_in_range_store_the_same(t *testing.T) {
	var pc PitchClass

	pc.SetAlteration(AlterationDoubleFlat)
	assert.Equal(t, AlterationDoubleFlat, pc.GetAlteration())

	pc.SetAlteration(AlterationFlat)
	assert.Equal(t, AlterationFlat, pc.GetAlteration())

	pc.SetAlteration(AlterationNatural)
	assert.Equal(t, AlterationNatural, pc.GetAlteration())

	pc.SetAlteration(AlterationSharp)
	assert.Equal(t, AlterationSharp, pc.GetAlteration())

	pc.SetAlteration(AlterationDoubleSharp)
	assert.Equal(t, AlterationDoubleSharp, pc.GetAlteration())
}

func Test_PitchClass_Alteration_When_value_is_smaller_than_limit_then_keep_min_value(t *testing.T) {
	var pc PitchClass

	for alt := Alteration(-12); alt <= Alteration(-2); alt++ {
		pc.SetAlteration(0)
		pc.SetAlteration(alt)
		assert.Equal(t, AlterationDoubleFlat, pc.GetAlteration())
	}
}

func Test_PitchClass_Alteration_When_value_is_greater_than_limit_then_keep_max_value(t *testing.T) {
	var pc PitchClass

	for alt := Alteration(2); alt <= Alteration(12); alt++ {
		pc.SetAlteration(0)
		pc.SetAlteration(alt)
		assert.Equal(t, AlterationDoubleSharp, pc.GetAlteration())
	}
}

func Test_PitchClass_Random_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[Step]bool)
	for i := 1; i <= 7*5; i++ {
		pc := RandomPitchClass()
		steps[pc.GetStep()] = true
	}
	for i := StepC; i <= StepB; i++ {
		assert.True(t, steps[i])
	}
}

func Test_PitchClass_Random_Alterations_should_never_be_double(t *testing.T) {
	alterations := make(map[Alteration]bool)
	for i := 1; i <= 3*5; i++ {
		pc := RandomPitchClass()
		alterations[pc.GetAlteration()] = true
	}
	for i := AlterationFlat; i <= AlterationSharp; i++ {
		assert.True(t, alterations[i])
	}
	for _, v := range []Alteration{AlterationDoubleFlat, AlterationDoubleSharp} {
		assert.False(t, alterations[v])
	}
}

func Test_PitchClass_ExtendedRandom_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[Step]bool)
	for i := 1; i <= 7*5; i++ {
		pc := ExtendedRandomPitchClass()
		steps[pc.GetStep()] = true
	}
	for i := StepC; i <= StepB; i++ {
		assert.True(t, steps[i])
	}
}

func Test_PitchClass_ExtendedRandom_Alterations_should_have_a_good_distribution(t *testing.T) {
	alterations := make(map[Alteration]bool)
	for i := 1; i <= 5*5; i++ {
		pc := ExtendedRandomPitchClass()
		alterations[pc.GetAlteration()] = true
	}
	for alt := AlterationDoubleFlat; alt <= AlterationDoubleSharp; alt++ {
		assert.True(t, alterations[alt])
	}
}

func Test_PitchClass_SimpleName_Without_alterations(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(0)
	names := []string{"C", "D", "E", "F", "G", "A", "B"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.Name()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_SimpleName_With_1_flat(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-1)
	names := []string{"Cb", "Db", "Eb", "Fb", "Gb", "Ab", "Bb"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.Name()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_SimpleName_With_2_flats(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-2)
	names := []string{"Cbb", "Dbb", "Ebb", "Fbb", "Gbb", "Abb", "Bbb"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.Name()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_SimpleName_With_1_sharp(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(1)
	names := []string{"C#", "D#", "E#", "F#", "G#", "A#", "B#"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.Name()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_SimpleName_With_2_sharps(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(2)
	names := []string{"C##", "D##", "E##", "F##", "G##", "A##", "B##"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.Name()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_PrettyName_Without_alterations(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(0)
	names := []string{"C", "D", "E", "F", "G", "A", "B"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.PrettyName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_PrettyName_With_1_flat(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-1)
	names := []string{"C♭", "D♭", "E♭", "F♭", "G♭", "A♭", "B♭"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.PrettyName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_PrettyName_With_2_flats(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-2)
	names := []string{"C𝄫", "D𝄫", "E𝄫", "F𝄫", "G𝄫", "A𝄫", "B𝄫"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.PrettyName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_PrettyName_With_1_sharp(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(1)
	names := []string{"C♯", "D♯", "E♯", "F♯", "G♯", "A♯", "B♯"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.PrettyName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_PrettyName_With_2_sharps(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(2)
	names := []string{"C𝄪", "D𝄪", "E𝄪", "F𝄪", "G𝄪", "A𝄪", "B𝄪"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.PrettyName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_FullName_Without_alterations(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(0)
	names := []string{"C", "D", "E", "F", "G", "A", "B"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_FullName_With_1_flat(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-1)
	names := []string{"C flat", "D flat", "E flat", "F flat", "G flat", "A flat", "B flat"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_FullName_With_2_flats(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(-2)
	names := []string{
		"C double flat", "D double flat", "E double flat", "F double flat",
		"G double flat", "A double flat", "B double flat",
	}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_FullName_With_1_sharp(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(1)
	names := []string{"C sharp", "D sharp", "E sharp", "F sharp", "G sharp", "A sharp", "B sharp"}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_FullName_With_2_sharps(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(2)
	names := []string{
		"C double sharp", "D double sharp", "E double sharp", "F double sharp",
		"G double sharp", "A double sharp", "B double sharp",
	}
	for i := StepC; i <= StepB; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitchClass_Parse(t *testing.T) {
	t.Run("Simple name", func(t *testing.T) {
		alterationTexts := []string{"bb", "b", "", "#", "##"}
		stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

		for ialt, alt := range alterationTexts {
			for istep, step := range stepTexts {
				text := step + alt
				t.Run(text, func(t *testing.T) {
					parsed := ParsePitchClass(text)

					assert.Equal(t, Step(istep+1), parsed.GetStep())
					assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
				})
			}
		}
	})
	t.Run("Pretty name", func(t *testing.T) {
		alterationTexts := []string{"𝄫", "♭", "♮", "♯", "𝄪"}
		stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

		for ialt, alt := range alterationTexts {
			for istep, step := range stepTexts {
				text := step + alt
				t.Run(text, func(t *testing.T) {
					parsed := ParsePitchClass(text)

					assert.Equal(t, Step(istep+1), parsed.GetStep())
					assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
				})
			}
		}
	})
	t.Run("Full name", func(t *testing.T) {
		alterationTexts := []string{"Double Flat", "Flat", "Natural", "Sharp", "Double Sharp"}
		stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

		for ialt, alt := range alterationTexts {
			for istep, step := range stepTexts {
				text := fmt.Sprintf("%s %s", step, alt)
				t.Run(text, func(t *testing.T) {
					parsed := ParsePitchClass(text)

					assert.Equal(t, Step(istep+1), parsed.GetStep())
					assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
				})
			}
		}
	})
}

func TestPitchClass_Next(t *testing.T) {
	testCases := []struct {
		pitchClass PitchClass
		next       PitchClass
	}{
		{
			pitchClass: PitchClassC(),
			next:       PitchClassCSharp(),
		},
		{
			pitchClass: PitchClassCSharp(),
			next:       PitchClassD(),
		},
		{
			pitchClass: PitchClassD(),
			next:       PitchClassDSharp(),
		},
		{
			pitchClass: PitchClassDSharp(),
			next:       PitchClassE(),
		},
		{
			pitchClass: PitchClassE(),
			next:       PitchClassF(),
		},
		{
			pitchClass: PitchClassESharp(),
			next:       PitchClassFSharp(),
		},
		{
			pitchClass: PitchClassF(),
			next:       PitchClassFSharp(),
		},
		{
			pitchClass: PitchClassFSharp(),
			next:       PitchClassG(),
		},
		{
			pitchClass: PitchClassG(),
			next:       PitchClassGSharp(),
		},
		{
			pitchClass: PitchClassGSharp(),
			next:       PitchClassA(),
		},
		{
			pitchClass: PitchClassA(),
			next:       PitchClassASharp(),
		},
		{
			pitchClass: PitchClassASharp(),
			next:       PitchClassB(),
		},
		{
			pitchClass: PitchClassB(),
			next:       PitchClassC(),
		},
		{
			pitchClass: PitchClassBSharp(),
			next:       PitchClassCSharp(),
		},
	}
	for _, tc := range testCases {
		t.Run(fmt.Sprintf("%s->%s", tc.pitchClass, tc.next), func(t *testing.T) {
			next := tc.pitchClass.Next()
			assert.Equal(t, tc.next.Name(), next.Name())
		})
	}
}
