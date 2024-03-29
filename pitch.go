package libmus

import (
	"fmt"
	"strconv"
)

const (
	MIN_OCTAVE = 0
	MAX_OCTAVE = 10
)

var octaves []int

func init() {
	for i := MIN_OCTAVE; i <= MAX_OCTAVE; i++ {
		octaves = append(octaves, i)
	}
}

type Pitch struct {
	pitchClass PitchClass
	octave     int
}

func NewPitch() Pitch {
	return Pitch{
		octave:     4,
		pitchClass: NewPitchClass(),
	}
}

func ParsePitch(text string) Pitch {
	pitch := NewPitch()
	head := text[0]

	for iname, name := range NAMES {
		if name == string(head) {
			pitch.pitchClass.SetStep(Step(iname + 1))
			break
		}
	}
	for i, head := range text[1:] {
		tail := text[i+1:]
		oct, err := strconv.Atoi(tail)
		if err == nil {
			pitch.octave = oct
		}

		switch head {
		case 'b', FLAT_SYMBOL:
			pitch.SetAlteration(pitch.GetAlteration() - 1)
		case '#', SHARP_SYMBOL:
			pitch.SetAlteration(pitch.GetAlteration() + 1)
		case DOUBLE_FLAT_SYMBOL:
			pitch.SetAlteration(pitch.GetAlteration() - 2)
		case DOUBLE_SHARP_SYMBOL:
			pitch.SetAlteration(pitch.GetAlteration() + 2)
		default:
			return pitch
		}
	}
	return pitch
}

func RandomPitch() Pitch {
	pitch := Pitch{}
	pitch.pitchClass = RandomPitchClass()
	pitch.octave = generateRandom(octaves)
	return pitch
}

func ExtendedRandomPitch() Pitch {
	pitch := Pitch{}
	pitch.pitchClass = ExtendedRandomPitchClass()
	pitch.octave = generateRandom(octaves)
	return pitch
}

func (p Pitch) GetStep() Step {
	return p.pitchClass.GetStep()
}

func (p *Pitch) SetStep(value Step) {
	p.pitchClass.SetStep(value)
}

func (p Pitch) GetAlteration() Alteration {
	return p.pitchClass.GetAlteration()
}

func (p *Pitch) SetAlteration(value Alteration) {
	p.pitchClass.SetAlteration(value)
}

func (p Pitch) GetOctave() int {
	return p.octave
}

func (p *Pitch) SetOctave(value int) {
	p.octave = truncateRange(value, MIN_OCTAVE, MAX_OCTAVE)
}

func (p Pitch) Name() string {
	octaveString := fmt.Sprint(p.octave)
	return p.pitchClass.Name() + octaveString
}

func (p Pitch) PrettyName() string {
	oct := SUPERSCRIPT_OCTAVES[p.octave]
	return p.pitchClass.PrettyName() + oct
}

func (p Pitch) FullName() string {
	octaveString := fmt.Sprint(p.octave)
	return p.pitchClass.FullName() + " " + octaveString
}

func (p Pitch) MIDINote() int {
	note := p.pitchClass.Number() + 12*p.octave
	return note
}
