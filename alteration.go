package libmus

import (
	"errors"
	"strings"
)

type Alteration int

const (
	AlterationDoubleFlat  Alteration = -2
	AlterationFlat        Alteration = -1
	AlterationNatural     Alteration = 0
	AlterationSharp       Alteration = 1
	AlterationDoubleSharp Alteration = 2
)

var ErrInvalidPitchAlteration = errors.New("invalid pitch alteration")

func (a Alteration) normalized() Alteration {
	return trunc(a, AlterationDoubleFlat, AlterationDoubleSharp)
}

func ParseAlteration(text string) (Alteration, error) {
	switch strings.ToLower(text) {
	case " flat", string(FLAT_SYMBOL), string('b'):
		return AlterationFlat, nil

	case " sharp", string(SHARP_SYMBOL), string('#'):
		return AlterationSharp, nil

	case " double flat", string(DOUBLE_FLAT_SYMBOL), "bb":
		return AlterationDoubleFlat, nil

	case " double sharp", string(DOUBLE_SHARP_SYMBOL), "##":
		return AlterationDoubleSharp, nil

	case "", " natural", string(NATURAL_SYMBOL):
		return AlterationNatural, nil
	default:
		return 0, errorWithDetails(ErrInvalidPitchAlteration, text)
	}
}
