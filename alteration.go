package libmus

type Alteration int

const (
	AlterationDoubleFlat  Alteration = -2
	AlterationFlat        Alteration = -1
	AlterationNatural     Alteration = 0
	AlterationSharp       Alteration = 1
	AlterationDoubleSharp Alteration = 2
)

func (a Alteration) Normalized() Alteration {
	return trunc(a, AlterationDoubleFlat, AlterationDoubleSharp)
}
