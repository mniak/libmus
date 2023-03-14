package libmus

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_PitchClass_Step_Constructor_should_set_initial_values(t *testing.T) {
	pc := NewPitchClass()

	assert.Equal(t, 1, pc.GetStep())
	assert.Equal(t, 0, pc.GetAlteration())
}

func Test_PitchClass_Step_Normal_values_should_work(t *testing.T) {
	var pc PitchClass

	pc.SetStep(1)
	assert.Equal(t, 1, pc.GetStep())

	pc.SetStep(2)
	assert.Equal(t, 2, pc.GetStep())

	pc.SetStep(3)
	assert.Equal(t, 3, pc.GetStep())

	pc.SetStep(4)
	assert.Equal(t, 4, pc.GetStep())

	pc.SetStep(5)
	assert.Equal(t, 5, pc.GetStep())

	pc.SetStep(6)
	assert.Equal(t, 6, pc.GetStep())

	pc.SetStep(7)
	assert.Equal(t, 7, pc.GetStep())
}

func Test_PitchClass_Step_Bigger_values_should_be_normalized(t *testing.T) {
	var pc PitchClass

	pc.SetStep(8)
	assert.Equal(t, 1, pc.GetStep())

	pc.SetStep(9)
	assert.Equal(t, 2, pc.GetStep())

	pc.SetStep(10)
	assert.Equal(t, 3, pc.GetStep())

	pc.SetStep(11)
	assert.Equal(t, 4, pc.GetStep())

	pc.SetStep(12)
	assert.Equal(t, 5, pc.GetStep())

	pc.SetStep(13)
	assert.Equal(t, 6, pc.GetStep())

	pc.SetStep(14)
	assert.Equal(t, 7, pc.GetStep())

	pc.SetStep(15)
	assert.Equal(t, 1, pc.GetStep())
}

func Test_PitchClass_Step_Attributing_zero_or_negative_should_do_nothing(t *testing.T) {
	var pc PitchClass

	for goodValue := 1; goodValue <= 6; goodValue++ {
		for badValue := -1; badValue >= -20; badValue-- {
			pc.SetStep(goodValue)
			pc.SetStep(badValue)
			assert.Equal(t, goodValue, pc.GetStep())
		}
	}
}

func Test_PitchClass_Alteration_When_value_is_in_range_store_the_same(t *testing.T) {
	var pc PitchClass

	pc.SetAlteration(-2)
	assert.Equal(t, -2, pc.GetAlteration())

	pc.SetAlteration(-1)
	assert.Equal(t, -1, pc.GetAlteration())

	pc.SetAlteration(0)
	assert.Equal(t, 0, pc.GetAlteration())

	pc.SetAlteration(1)
	assert.Equal(t, 1, pc.GetAlteration())

	pc.SetAlteration(2)
	assert.Equal(t, 2, pc.GetAlteration())
}

func Test_PitchClass_Alteration_When_value_is_smaller_than_limit_then_keep_min_value(t *testing.T) {
	var pc PitchClass

	for v := -12; v <= -2; v++ {
		pc.SetAlteration(0)
		pc.SetAlteration(v)
		assert.Equal(t, -2, pc.GetAlteration())
	}
}

func Test_PitchClass_Alteration_When_value_is_greater_than_limit_then_keep_max_value(t *testing.T) {
	var pc PitchClass

	for v := 2; v <= 12; v++ {
		pc.SetAlteration(0)
		pc.SetAlteration(v)
		assert.Equal(t, 2, pc.GetAlteration())
	}
}

func Test_PitchClass_Random_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[int]bool)
	for i := 1; i <= 7*5; i++ {
		pc := RandomPitchClass()
		steps[pc.GetStep()] = true
	}
	for i := 1; i <= 7; i++ {
		assert.True(t, steps[i])
	}
}

func Test_PitchClass_Random_Alterations_should_never_be_double(t *testing.T) {
	alterations := make(map[int]bool)
	for i := 1; i <= 3*5; i++ {
		pc := RandomPitchClass()
		alterations[pc.GetAlteration()] = true
	}
	for i := -1; i <= 1; i++ {
		assert.True(t, alterations[i])
	}
	for _, v := range []int{-2, 2} {
		assert.False(t, alterations[v])
	}
}

func Test_PitchClass_ExtendedRandom_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[int]bool)
	for i := 1; i <= 7*5; i++ {
		pc := ExtendedRandomPitchClass()
		steps[pc.GetStep()] = true
	}
	for i := 1; i <= 7; i++ {
		assert.True(t, steps[i])
	}
}

func Test_PitchClass_ExtendedRandom_Alterations_should_have_a_good_distribution(t *testing.T) {
	alterations := make(map[int]bool)
	for i := 1; i <= 5*5; i++ {
		pc := ExtendedRandomPitchClass()
		alterations[pc.GetAlteration()] = true
	}
	for i := -2; i <= 2; i++ {
		assert.True(t, alterations[i])
	}
}

func Test_PitchClass_SimpleName_Without_alterations(t *testing.T) {
	var pc PitchClass
	pc.SetAlteration(0)
	names := []string{"C", "D", "E", "F", "G", "A", "B"}
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
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
	for i := 1; i <= 7; i++ {
		pc.SetStep(i)
		expected := names[i-1]
		actual := pc.FullName()
		assert.Equal(t, expected, actual)
	}
}

func Test_PitchClass_Parse_Simple_name(t *testing.T) {
	alterationTexts := []string{"bb", "b", "", "#", "##"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for ialt := 0; ialt < len(alterationTexts); ialt++ {
		alt := alterationTexts[ialt]

		for istep := 0; istep < len(stepTexts); istep++ {
			step := stepTexts[istep]

			text := step + alt
			parsed := ParsePitchClass(text)

			assert.Equal(t, istep+1, parsed.GetStep())
			assert.Equal(t, ialt-2, parsed.GetAlteration())
		}
	}
}

func Test_PitchClass_Parse_Pretty_name(t *testing.T) {
	alterationTexts := []string{"𝄫", "♭", "", "♯", "𝄪"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for ialt := 0; ialt < len(alterationTexts); ialt++ {
		alt := alterationTexts[ialt]

		for istep := 0; istep < len(stepTexts); istep++ {
			step := stepTexts[istep]

			text := step + alt
			parsed := ParsePitchClass(text)

			assert.Equal(t, istep+1, parsed.GetStep())
			assert.Equal(t, ialt-2, parsed.GetAlteration())
		}
	}
}
