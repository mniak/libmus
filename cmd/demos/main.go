package main

import (
	"fmt"

	"github.com/mniak/libmus"
)

func main() {
	// cmd := cobra.Command{
	// 	Use: "demos",
	// }
	// cmd.AddCommand()
	// cmd.Execute()

	p := libmus.RandomPitch()
	fmt.Printf("Random Pitch: %s\n", p)
	fmt.Printf("  Name       : %s\n", p.Name())
	fmt.Printf("  Full name  : %s\n", p.FullName())
	fmt.Printf("  Pretty name: %s\n", p.PrettyName())
	fmt.Printf("  Octave     : %d\n", p.GetOctave())
	fmt.Println()

	pc := p.Class()
	fmt.Printf("Pitch Class: %s\n", pc)
	fmt.Printf("  Name       : %s\n", pc.Name())
	fmt.Printf("  Full name  : %s\n", pc.FullName())
	fmt.Printf("  Pretty name: %s\n", pc.PrettyName())
	fmt.Printf("  Number     : %d\n", pc.Number())
	fmt.Printf("  Next       : %s\n", pc.Next())
	fmt.Printf("  Previous   : %s\n", pc.Previous())
	fmt.Println()
}
