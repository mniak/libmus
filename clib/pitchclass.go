package main

/*
typedef void* PitchClass;
*/
import "C"

import (
	"runtime/cgo"

	"github.com/mniak/libmus"
)

func PitchClass_ToPointer(pc *libmus.PitchClass) C.PitchClass {
	return C.PitchClass(cgo.NewHandle(pc))
}

func PitchClass_FromPointer(ptr C.PitchClass) *libmus.PitchClass {
	return cgo.Handle(ptr).Value().(*libmus.PitchClass)
}

//export PitchClass_New
func PitchClass_New() C.PitchClass {
	pc := libmus.PitchClass{}
	pc.SetStep(7)
	pc.SetAlteration(1)
	return PitchClass_ToPointer(&pc)
}

//export PitchClass_PrettyName
func PitchClass_PrettyName(ptr C.PitchClass) *C.char {
	pc := PitchClass_FromPointer(ptr)
	return C.CString(pc.PrettyName())
}
