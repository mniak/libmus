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
	return C.PitchClass(cgo.NewHandle(result))
}

//export GetStep
func GetStep(h C.PitchClass) int {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.GetStep()
	return result
}

//export SetStep
func SetStep(h C.PitchClass, value int) {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	this.SetStep(value)
}

//export GetAlteration
func GetAlteration(h C.PitchClass) int {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.GetAlteration()
	return result
}

//export SetAlteration
func SetAlteration(h C.PitchClass, value int) {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	this.SetAlteration(value)
}

//export RandomPitchClass
func RandomPitchClass() C.PitchClass {
	result := libmus.RandomPitchClass()
	return C.PitchClass(cgo.NewHandle(result))
}

//export ExtendedRandomPitchClass
func ExtendedRandomPitchClass() C.PitchClass {
	result := libmus.ExtendedRandomPitchClass()
	return C.PitchClass(cgo.NewHandle(result))
}

//export ParsePitchClass
func ParsePitchClass(text *C.char) C.PitchClass {
	result := libmus.ParsePitchClass(text)
	return C.PitchClass(cgo.NewHandle(result))
}

//export Name
func Name(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.Name()
	return C.CString(result)
}

//export PrettyName
func PrettyName(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.PrettyName()
	return C.CString(result)
}

//export FullName
func FullName(h C.PitchClass) *C.char {
	this := cgo.Handle(h).Value().(*libmus.PitchClass)
	result := this.FullName()
	return C.CString(result)
}
