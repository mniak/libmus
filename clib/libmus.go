package main

import "C"
import "fmt"

//export MyFunction
func MyFunction(arg1, arg2 int, arg3 *C.char) int64 {
	fmt.Println(C.GoString(arg3))
	return 0
}

func main() {}

//export PitchClass
type PitchClass struct{}
