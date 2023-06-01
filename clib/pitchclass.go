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
	result := libmus.NewPitchClass()
	resultC := C.PitchClass(cgo.NewHandle(&result))
	return resultC
}
//export PitchClass_GetStep
func PitchClass_GetStep(handle C.PitchClass) int {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	result := this.GetStep()
	return result
}
//export PitchClass_SetStep
func PitchClass_SetStep(handle C.PitchClass, value int) {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	this.SetStep(value)
}
//export PitchClass_GetAlteration
func PitchClass_GetAlteration(handle C.PitchClass) int {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	result := this.GetAlteration()
	return result
}
//export PitchClass_SetAlteration
func PitchClass_SetAlteration(handle C.PitchClass, value int) {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	this.SetAlteration(value)
}
//export RandomPitchClass
func RandomPitchClass() C.PitchClass {
	result := libmus.RandomPitchClass()
	resultC := C.PitchClass(cgo.NewHandle(&result))
	return resultC
}
//export ExtendedRandomPitchClass
func ExtendedRandomPitchClass() C.PitchClass {
	result := libmus.ExtendedRandomPitchClass()
	resultC := C.PitchClass(cgo.NewHandle(&result))
	return resultC
}
//export ParsePitchClass
func ParsePitchClass(text *C.char) C.PitchClass {
	result := libmus.ParsePitchClass(C.GoString(text))
	resultC := C.PitchClass(cgo.NewHandle(&result))
	return resultC
}
//export PitchClass_Name
func PitchClass_Name(handle C.PitchClass) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	result := this.Name()
	resultC := C.CString(result)
	return resultC
}
//export PitchClass_PrettyName
func PitchClass_PrettyName(handle C.PitchClass) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	result := this.PrettyName()
	resultC := C.CString(result)
	return resultC
}
//export PitchClass_FullName
func PitchClass_FullName(handle C.PitchClass) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.PitchClass)
	result := this.FullName()
	resultC := C.CString(result)
	return resultC
}

