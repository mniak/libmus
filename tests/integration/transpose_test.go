package integration_tests

import (
	"fmt"
	"testing"

	"github.com/mniak/libmus"
	"github.com/stretchr/testify/assert"
)

func TestTranspose(t *testing.T) {
	pitch := libmus.Pitch{
		PitchClass: libmus.PitchClassC(),
		Octave:     4,
	}
	assert.Equal(t, "C4", fmt.Sprint(pitch))

	third := pitch.Transpose(libmus.MajorThird())
	assert.Equal(t, "E4", fmt.Sprint(third))

	fifth := pitch.Transpose(libmus.Fifth())
	assert.Equal(t, "G4", fmt.Sprint(fifth))

	octave := pitch.Transpose(libmus.Octave())
	assert.Equal(t, "C5", fmt.Sprint(octave))
}
