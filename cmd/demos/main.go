package main

import (
	"fmt"

	"github.com/mniak/libmus"
)

func main() {
	p := libmus.Pitch{
		PitchClass: libmus.PitchClassC(),
		Octave:     4,
	}
	fmt.Println(p)

	i := libmus.MajorThird()
	p2 := p.ApplyInterval(i)
	fmt.Println(p2)
}
