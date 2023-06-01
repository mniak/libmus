package clib

import (
	"github.com/magefile/mage/sh"
)

// Builds the C library
func Build() error {
	err := sh.RunV(
		"go", "build",
		"-buildmode=c-shared",
		"-tags=clib",
		"-o", "clib/libmus.a",
		"./clib",
	)
	if err != nil {
		return err
	}
	return nil
}
