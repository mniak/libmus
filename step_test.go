package libmus

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStep_Previous(t *testing.T) {
	testCases := []struct {
		step     Step
		previous Step
	}{
		{
			step:     StepC,
			previous: StepB,
		},
		{
			step:     StepB,
			previous: StepA,
		},
		{
			step:     StepA,
			previous: StepG,
		},
		{
			step:     StepG,
			previous: StepF,
		},
		{
			step:     StepF,
			previous: StepE,
		},
		{
			step:     StepE,
			previous: StepD,
		},
		{
			step:     StepD,
			previous: StepC,
		},
	}
	for _, tc := range testCases {
		t.Run(fmt.Sprintf("%s->%s", tc.step, tc.previous), func(t *testing.T) {
			previous := tc.step.Previous()
			assert.Equal(t, tc.previous.Name(), previous.Name())
		})
	}
}

func TestStep_Next(t *testing.T) {
	testCases := []struct {
		step Step
		next Step
	}{
		{
			step: StepB,
			next: StepC,
		},
		{
			step: StepA,
			next: StepB,
		},
		{
			step: StepG,
			next: StepA,
		},
		{
			step: StepF,
			next: StepG,
		},
		{
			step: StepE,
			next: StepF,
		},
		{
			step: StepD,
			next: StepE,
		},
		{
			step: StepC,
			next: StepD,
		},
	}
	for _, tc := range testCases {
		t.Run(fmt.Sprintf("%s->%s", tc.step, tc.next), func(t *testing.T) {
			next := tc.step.Next()
			assert.Equal(t, tc.next.Name(), next.Name())
		})
	}
}
