package main

/*
typedef void* PitchClass;
*/
import "C"

import (
	"fmt"
	"runtime/cgo"
)

func main() {}

type PitchClass struct {
	value string
}

func (pc *PitchClass) Pointer() C.PitchClass {
	return C.PitchClass(cgo.NewHandle(pc))
}

func PitchClass_FromPointer(ptr C.PitchClass) *PitchClass {
	return cgo.Handle(ptr).Value().(*PitchClass)
}

//export PitchClass_New
func PitchClass_New() C.PitchClass {
	pc := PitchClass{
		value: "this is its value",
	}
	return pc.Pointer()
}

//export PitchClass_PrintValue
func PitchClass_PrintValue(ptr C.PitchClass) {
	pc := PitchClass_FromPointer(ptr)
	fmt.Println(pc.value)
}
