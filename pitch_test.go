package libmus

import (
	"fmt"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPitch_Assert_initial_values_from_constructor_are_being_set(t *testing.T) {
	pitch := NewPitch()

	assert.Equal(t, StepC, pitch.GetStep())
	assert.Equal(t, AlterationNatural, pitch.GetAlteration())
	assert.Equal(t, 4, pitch.GetOctave())
}

func TestPitch_Step_Normal_values_should_work(t *testing.T) {
	pitch := NewPitch()

	pitch.SetStep(StepC)
	assert.Equal(t, StepC, pitch.GetStep())

	pitch.SetStep(StepD)
	assert.Equal(t, StepD, pitch.GetStep())

	pitch.SetStep(StepE)
	assert.Equal(t, StepE, pitch.GetStep())

	pitch.SetStep(StepF)
	assert.Equal(t, StepF, pitch.GetStep())

	pitch.SetStep(StepG)
	assert.Equal(t, StepG, pitch.GetStep())

	pitch.SetStep(StepA)
	assert.Equal(t, StepA, pitch.GetStep())

	pitch.SetStep(StepB)
	assert.Equal(t, StepB, pitch.GetStep())
}

func TestPitch_Step_Bigger_values_should_be_normalized(t *testing.T) {
	pitch := NewPitch()

	pitch.SetStep(8)
	assert.Equal(t, StepC, pitch.GetStep())

	pitch.SetStep(9)
	assert.Equal(t, StepD, pitch.GetStep())

	pitch.SetStep(10)
	assert.Equal(t, StepE, pitch.GetStep())

	pitch.SetStep(11)
	assert.Equal(t, StepF, pitch.GetStep())

	pitch.SetStep(12)
	assert.Equal(t, StepG, pitch.GetStep())

	pitch.SetStep(13)
	assert.Equal(t, StepA, pitch.GetStep())

	pitch.SetStep(14)
	assert.Equal(t, StepB, pitch.GetStep())

	pitch.SetStep(15)
	assert.Equal(t, StepC, pitch.GetStep())
}

func TestPitch_Step_Zero_or_negative_values_should_do_nothing(t *testing.T) {
	pitch := NewPitch()

	for goodValue := StepC; goodValue <= StepB; goodValue++ {
		for badValue := Step(-1); badValue >= Step(-20); badValue-- {
			pitch.SetStep(goodValue)
			pitch.SetStep(badValue)
			assert.Equal(t, goodValue, pitch.GetStep())
		}
	}
}

func TestPitch_Alteration_When_value_is_in_range_store_the_same(t *testing.T) {
	pitch := NewPitch()

	pitch.SetAlteration(AlterationDoubleFlat)
	assert.Equal(t, AlterationDoubleFlat, pitch.GetAlteration())

	pitch.SetAlteration(AlterationFlat)
	assert.Equal(t, AlterationFlat, pitch.GetAlteration())

	pitch.SetAlteration(AlterationNatural)
	assert.Equal(t, AlterationNatural, pitch.GetAlteration())

	pitch.SetAlteration(AlterationSharp)
	assert.Equal(t, AlterationSharp, pitch.GetAlteration())

	pitch.SetAlteration(AlterationDoubleSharp)
	assert.Equal(t, AlterationDoubleSharp, pitch.GetAlteration())
}

func TestPitch_Alteration_When_value_is_smaller_than_limit_keep_min_value(t *testing.T) {
	pitch := NewPitch()

	for alt := Alteration(-12); alt <= Alteration(-2); alt++ {
		pitch.SetAlteration(0)
		pitch.SetAlteration(alt)
		assert.Equal(t, AlterationDoubleFlat, pitch.GetAlteration())
	}
}

func TestPitch_Alteration_When_value_is_greater_than_limit_keep_max_value(t *testing.T) {
	pitch := NewPitch()

	for alt := Alteration(2); alt <= Alteration(12); alt++ {
		pitch.SetAlteration(0)
		pitch.SetAlteration(alt)
		assert.Equal(t, AlterationDoubleSharp, pitch.GetAlteration())
	}
}

func TestPitch_Octave_When_value_is_in_range_store_the_same(t *testing.T) {
	pitch := NewPitch()
	for v := 0; v <= 10; v++ {
		pitch.SetOctave(v)
		assert.Equal(t, v, pitch.GetOctave())
	}
}

func TestPitch_Octave_When_value_is_smaller_than_limit_keep_min_value(t *testing.T) {
	pitch := NewPitch()

	for v := -12; v <= 0; v++ {
		pitch.SetOctave(0)
		pitch.SetOctave(v)
		assert.Equal(t, 0, pitch.GetOctave())
	}
}

func TestPitch_Octave_When_value_is_greater_than_limit_keep_max_value(t *testing.T) {
	pitch := NewPitch()

	for v := 10; v <= 25; v++ {
		pitch.SetOctave(0)
		pitch.SetOctave(v)
		assert.Equal(t, 10, pitch.GetOctave())
	}
}

func TestPitch_Random_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[Step]bool)
	for i := 1; i <= 7*5; i++ {
		pitch := RandomPitch()
		steps[pitch.GetStep()] = true
	}
	for i := StepC; i <= StepB; i++ {
		assert.True(t, steps[i])
	}
}

func TestPitch_Random_Alterations_should_never_be_double(t *testing.T) {
	alterations := make(map[Alteration]bool)
	for i := 1; i <= 5*5; i++ {
		pitch := RandomPitch()
		alterations[pitch.GetAlteration()] = true
	}
	for i := AlterationFlat; i <= AlterationSharp; i++ {
		assert.True(t, alterations[i])
	}
	fmt.Println(alterations)
	for _, v := range []Alteration{AlterationDoubleFlat, AlterationDoubleSharp} {
		assert.False(t, alterations[v])
	}
}

func TestPitch_Random_Octaves_should_have_a_good_distribution(t *testing.T) {
	octaves := make(map[int]bool)
	for i := 1; i <= 10*5; i++ {
		pitch := RandomPitch()
		octaves[pitch.GetOctave()] = true
	}
	for i := 0; i <= 10; i++ {
		assert.True(t, octaves[i])
	}
}

func TestPitch_ExtendedRandom_Steps_should_have_a_good_distribution(t *testing.T) {
	steps := make(map[Step]bool)
	for i := 1; i <= 7*5; i++ {
		pitch := ExtendedRandomPitch()
		steps[pitch.GetStep()] = true
	}
	for i := StepC; i <= StepB; i++ {
		t.Run(fmt.Sprint(i), func(t *testing.T) {
			assert.True(t, steps[i])
		})
	}
}

func TestPitch_ExtendedRandom_Alterations_should_have_a_good_distribution(t *testing.T) {
	alterations := make(map[Alteration]bool)
	for i := 1; i <= 5*5; i++ {
		pitch := ExtendedRandomPitch()
		alterations[pitch.GetAlteration()] = true
	}
	for i := AlterationDoubleFlat; i <= AlterationDoubleSharp; i++ {
		assert.True(t, alterations[i])
	}
}

func TestPitch_ExtendedRandom_Octaves_should_have_a_good_distribution(t *testing.T) {
	octaves := make(map[int]bool)
	for i := 1; i <= 10*5; i++ {
		pitch := ExtendedRandomPitch()
		octaves[pitch.GetOctave()] = true
	}
	for i := 0; i <= 10; i++ {
		assert.True(t, octaves[i])
	}
}

func TestPitch_SimpleName_Without_alterations(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(0)
	names := []string{"C?", "D?", "E?", "F?", "G?", "A?", "B?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for s := StepC; s <= StepB; s++ {
			pitch.SetStep(s)
			expected := strings.ReplaceAll(names[s-1], "?", fmt.Sprint(pitch.GetOctave()))
			actual := pitch.Name()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_SimpleName_With_1_flat(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-1)
	names := []string{"Cb?", "Db?", "Eb?", "Fb?", "Gb?", "Ab?", "Bb?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for s := StepC; s <= StepB; s++ {
			pitch.SetStep(s)
			expected := strings.ReplaceAll(names[s-1], "?", fmt.Sprint(pitch.GetOctave()))
			actual := pitch.Name()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_SimpleName_With_2_flats(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-2)
	names := []string{"Cbb?", "Dbb?", "Ebb?", "Fbb?", "Gbb?", "Abb?", "Bbb?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
			actual := pitch.Name()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_SimpleName_With_1_sharp(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(1)
	names := []string{"C#?", "D#?", "E#?", "F#?", "G#?", "A#?", "B#?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
			actual := pitch.Name()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_SimpleName_With_2_sharps(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(2)
	names := []string{"C##?", "D##?", "E##?", "F##?", "G##?", "A##?", "B##?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
			actual := pitch.Name()
			assert.Equal(t, expected, actual)
		}
	}
}

var superscripts = []string{"⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "¹⁰"}

func TestPitch_PrettyName_Without_alterations(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(0)
	names := []string{"C?", "D?", "E?", "F?", "G?", "A?", "B?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", superscripts[pitch.GetOctave()])
			actual := pitch.PrettyName()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_PrettyName_With_1_flat(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-1)
	names := []string{"C♭?", "D♭?", "E♭?", "F♭?", "G♭?", "A♭?", "B♭?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", superscripts[pitch.GetOctave()])
			actual := pitch.PrettyName()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_PrettyName_With_2_flats(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-2)
	names := []string{"C𝄫?", "D𝄫?", "E𝄫?", "F𝄫?", "G𝄫?", "A𝄫?", "B𝄫?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", superscripts[pitch.GetOctave()])
			actual := pitch.PrettyName()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_PrettyName_With_1_sharp(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(1)
	names := []string{"C♯?", "D♯?", "E♯?", "F♯?", "G♯?", "A♯?", "B♯?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", superscripts[pitch.GetOctave()])
			actual := pitch.PrettyName()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_PrettyName_With_2_sharps(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(2)
	names := []string{"C𝄪?", "D𝄪?", "E𝄪?", "F𝄪?", "G𝄪?", "A𝄪?", "B𝄪?"}
	for o := 0; o <= 10; o++ {
		pitch.SetOctave(o)
		for i := StepC; i <= StepB; i++ {
			pitch.SetStep(i)
			expected := strings.ReplaceAll(names[i-1], "?", superscripts[pitch.GetOctave()])
			actual := pitch.PrettyName()
			assert.Equal(t, expected, actual)
		}
	}
}

func TestPitch_FullName_Without_alterations(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(0)
	names := []string{"C ?", "D ?", "E ?", "F ?", "G ?", "A ?", "B ?"}
	for i := StepC; i <= StepB; i++ {
		pitch.SetStep(i)
		expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
		actual := pitch.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitch_FullName_With_1_flat(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-1)
	names := []string{
		"C flat ?", "D flat ?", "E flat ?", "F flat ?",
		"G flat ?", "A flat ?", "B flat ?",
	}
	for i := StepC; i <= StepB; i++ {
		pitch.SetStep(i)
		expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
		actual := pitch.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitch_FullName_With_2_flats(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(-2)
	names := []string{
		"C double flat ?", "D double flat ?", "E double flat ?", "F double flat ?",
		"G double flat ?", "A double flat ?", "B double flat ?",
	}
	for i := StepC; i <= StepB; i++ {
		pitch.SetStep(i)
		expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
		actual := pitch.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitch_FullName_With_1_sharp(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(1)
	names := []string{
		"C sharp ?", "D sharp ?", "E sharp ?", "F sharp ?",
		"G sharp ?", "A sharp ?", "B sharp ?",
	}
	for i := StepC; i <= StepB; i++ {
		pitch.SetStep(i)
		expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
		actual := pitch.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitch_FullName_With_2_sharps(t *testing.T) {
	pitch := NewPitch()
	pitch.SetAlteration(2)
	names := []string{
		"C double sharp ?", "D double sharp ?", "E double sharp ?", "F double sharp ?",
		"G double sharp ?", "A double sharp ?", "B double sharp ?",
	}

	for i := StepC; i <= StepB; i++ {
		pitch.SetStep(i)
		expected := strings.ReplaceAll(names[i-1], "?", fmt.Sprint(pitch.GetOctave()))
		actual := pitch.FullName()
		assert.Equal(t, expected, actual)
	}
}

func TestPitch_Parse_without_octave_Simple_name_without_octave(t *testing.T) {
	alterationTexts := []string{"bb", "b", "", "#", "##"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for ialt, alt := range alterationTexts {
		for istep, step := range stepTexts {

			text := step + alt
			t.Run(text, func(t *testing.T) {
				parsed := ParsePitch(text)

				assert.Equal(t, Step(istep+1), parsed.GetStep())
				assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
				assert.Equal(t, 4, parsed.GetOctave())
			})
		}
	}
}

func TestPitch_Parse_without_octave_Pretty_name(t *testing.T) {
	alterationTexts := []string{"𝄫", "♭", "", "♯", "𝄪"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for ialt, alt := range alterationTexts {
		for istep, step := range stepTexts {

			text := step + alt
			t.Run(text, func(t *testing.T) {
				parsed := ParsePitch(text)

				assert.Equal(t, Step(istep+1), parsed.GetStep())
				assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
				assert.Equal(t, 4, parsed.GetOctave())
			})
		}
	}
}

func TestPitch_Parse_with_octave_Simple_name(t *testing.T) {
	alterationTexts := []string{"bb", "b", "", "#", "##"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for oct := 0; oct <= 10; oct++ {
		for ialt, alt := range alterationTexts {
			for istep, step := range stepTexts {

				octaveString := fmt.Sprint(oct)
				text := step + alt + octaveString
				t.Run(text, func(t *testing.T) {
					parsed := ParsePitch(text)

					assert.Equal(t, Step(istep+1), parsed.GetStep())
					assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
					assert.Equal(t, oct, parsed.GetOctave())
				})
			}
		}
	}
}

func TestPitch_Parse_with_octave_Pretty_name(t *testing.T) {
	alterationTexts := []string{"𝄫", "♭", "", "♯", "𝄪"}
	stepTexts := []string{"C", "D", "E", "F", "G", "A", "B"}

	for oct := 0; oct <= 10; oct++ {
		for ialt, alt := range alterationTexts {
			for istep, step := range stepTexts {

				octaveString := fmt.Sprint(oct)
				text := step + alt + octaveString
				t.Run(text, func(t *testing.T) {
					parsed := ParsePitch(text)

					assert.Equal(t, Step(istep+1), parsed.GetStep())
					assert.Equal(t, Alteration(ialt-2), parsed.GetAlteration())
					assert.Equal(t, oct, parsed.GetOctave())
				})
			}
		}
	}
}
