package libmus

type (
	QualityM string
	QualityP string
)

const (
	Augmented  QualityP = "augmented"
	Major      QualityM = "major"
	Perfect    QualityP = "perfect"
	Minor      QualityM = "minor"
	Diminished QualityP = "diminished"
)

// type Chord interface {
// 	Name() string
// 	Pitches() []Pitch
// }

type Triad struct {
	Root  PitchClass
	Third QualityM
	Fifth QualityP
}

func (t Triad) ToConcreteSimple(octave int) []Pitch {
	root := t.Root.OnOctave(octave)
	scale := CMajorScale()
	third := scale.Transpose(root, Interval{Number: 3})
	fifth := scale.Transpose(root, Interval{Number: 5})
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
