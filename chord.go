package libmus

type QualityM string

const (
	MajorThird QualityM = "major"
	MinorThird QualityM = "minor"
)

type QualityP string

const (
	AugmentedFifth  QualityP = "augmented"
	PerfectFifth    QualityP = "perfect"
	DiminishedFifth QualityP = "diminished"
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
