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

	// p := libmus.RandomPitch()
	// fmt.Printf("Random Pitch: %s\n", p)
	// fmt.Printf("  Name       : %s\n", p.Name())
	// fmt.Printf("  Full name  : %s\n", p.FullName())
	// fmt.Printf("  Pretty name: %s\n", p.PrettyName())
	// fmt.Println()

	// pc := p.Class()
	pc := libmus.PitchClassC()
	fmt.Printf("Pitch Class: %s\n", pc)
	fmt.Printf("  Name       : %s\n", pc.Name())
	fmt.Printf("  Full name  : %s\n", pc.FullName())
	fmt.Printf("  Pretty name: %s\n", pc.PrettyName())
	fmt.Printf("  Next: %s\n", pc.Next())
	fmt.Println()
}
