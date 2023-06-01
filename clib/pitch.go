//go:build clib

package main

import (
	"runtime/cgo"
	"github.com/mniak/libmus"
)
// typedef void* Pitch;
import "C"
//export NewPitch
func NewPitch() C.Pitch {
	result := libmus.NewPitch()
	resultC := C.Pitch(cgo.NewHandle(&result))
	return resultC
}
//export ParsePitch
func ParsePitch(text *C.char) C.Pitch {
	result := libmus.ParsePitch(C.GoString(text))
	resultC := C.Pitch(cgo.NewHandle(&result))
	return resultC
}
//export RandomPitch
func RandomPitch() C.Pitch {
	result := libmus.RandomPitch()
	resultC := C.Pitch(cgo.NewHandle(&result))
	return resultC
}
//export ExtendedRandomPitch
func ExtendedRandomPitch() C.Pitch {
	result := libmus.ExtendedRandomPitch()
	resultC := C.Pitch(cgo.NewHandle(&result))
	return resultC
}
//export Pitch_GetStep
func Pitch_GetStep(handle C.Pitch) int {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.GetStep()
	return result
}
//export Pitch_SetStep
func Pitch_SetStep(handle C.Pitch, value int) {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	this.SetStep(value)
}
//export Pitch_GetAlteration
func Pitch_GetAlteration(handle C.Pitch) int {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.GetAlteration()
	return result
}
//export Pitch_SetAlteration
func Pitch_SetAlteration(handle C.Pitch, value int) {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	this.SetAlteration(value)
}
//export Pitch_GetOctave
func Pitch_GetOctave(handle C.Pitch) int {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.GetOctave()
	return result
}
//export Pitch_SetOctave
func Pitch_SetOctave(handle C.Pitch, value int) {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	this.SetOctave(value)
}
//export Pitch_Name
func Pitch_Name(handle C.Pitch) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.Name()
	resultC := C.CString(result)
	return resultC
}
//export Pitch_PrettyName
func Pitch_PrettyName(handle C.Pitch) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.PrettyName()
	resultC := C.CString(result)
	return resultC
}
//export Pitch_FullName
func Pitch_FullName(handle C.Pitch) *C.char {
	this := cgo.Handle(handle).Value().(*libmus.Pitch)
	result := this.FullName()
	resultC := C.CString(result)
	return resultC
}

