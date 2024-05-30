package libmus

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInterval_Semitones(t *testing.T) {
	P1 := Unison()
	assert.Equal(t, 0, P1.Semitones())

	M3 := MajorThird()
	assert.Equal(t, 4, M3.Semitones())

	P8 := Octave()
	assert.Equal(t, 12, P8.Semitones())
}
