//go:build clib

package main

import (
	"runtime/cgo"
	"github.com/mniak/libmus"
)
// typedef void* PitchClass;
import "C"
//export NewPitchClass
func NewPitchClass() C.PitchClass {
	result := C.PitchClass(cgo.NewHandle(libmus.NewPitchClass()))
	return result
}
//export PitchClass_GetStep
func PitchClass_GetStep(h C.PitchClass) int {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.GetStep()
	return result
}
//export PitchClass_SetStep
func PitchClass_SetStep(h C.PitchClass, value int) {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	this.SetStep(value)
}
//export PitchClass_GetAlteration
func PitchClass_GetAlteration(h C.PitchClass) int {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.GetAlteration()
	return result
}
//export PitchClass_SetAlteration
func PitchClass_SetAlteration(h C.PitchClass, value int) {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	this.SetAlteration(value)
}
//export RandomPitchClass
func RandomPitchClass() C.PitchClass {
	result := C.PitchClass(cgo.NewHandle(libmus.RandomPitchClass()))
	return result
}
//export ExtendedRandomPitchClass
func ExtendedRandomPitchClass() C.PitchClass {
	result := C.PitchClass(cgo.NewHandle(libmus.ExtendedRandomPitchClass()))
	return result
}
//export ParsePitchClass
func ParsePitchClass(text *C.char) C.PitchClass {
	result := C.PitchClass(cgo.NewHandle(libmus.ParsePitchClass(C.GoString(text))))
	return result
}
//export PitchClass_Name
func PitchClass_Name(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := C.CString(this.Name())
	return result
}
//export PitchClass_PrettyName
func PitchClass_PrettyName(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := C.CString(this.PrettyName())
	return result
}
//export PitchClass_FullName
func PitchClass_FullName(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := C.CString(this.FullName())
	return result
}

