package libmus

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInterval_Semitones(t *testing.T) {
	testCases := []struct {
		name     string
		interval Interval
		expected int
	}{
		{
			name:     "Unison",
			interval: Unison(),
			expected: 0,
		},
		{
			name:     "Major third",
			interval: MajorThird(),
			expected: 4,
		},
		{
			name:     "Octave",
			interval: Octave(),
			expected: 12,
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			result := tc.interval.Semitones()
			assert.Equal(t, tc.expected, result)

			descendingSemitones := tc.interval.Descending().Semitones()
			assert.Equal(t, -tc.expected, descendingSemitones)
		})
	}
}
