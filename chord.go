package libmus

// type Chord interface {
// 	Name() string
// 	Pitches() []Pitch
// }

type Triad struct {
	Root  PitchClass
	Third Quality
	Fifth Quality
}

func (t Triad) ToConcreteSimple(octave int) []Pitch {
	root := t.Root.OnOctave(octave)
	scale := CMajorScale()
	third := scale.Transpose(root, Interval{Degree: 3})
	fifth := scale.Transpose(root, Interval{Degree: 5})
	return []Pitch{root, third, fifth}
}

// func Triad(
// 	root PitchClass,
// 	third QualityM,
// 	fifth QualityP,
// ) triad {
// 	return triad{
// 		Root:  root,
// 		Third: third,
// 		Fifth: fifth,
// 	}
// }
