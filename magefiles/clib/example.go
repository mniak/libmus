package clib

import (
	"path/filepath"

	"github.com/magefile/mage/mg"
	"github.com/magefile/mage/sh"
)

// Builds the C library example
func Example_build() error {
	mg.Deps(Build)
	filenames, err := filepath.Glob("clib/example/*.c")
	if err != nil {
		return err
	}

	var args []string
	args = append(args, filenames...)
	args = append(args, "-L", "clib")
	args = append(args, "-lmus")
	args = append(args, "-o", "clib/example/example.out")
	err = sh.RunV("gcc", args...)
	if err != nil {
		return err
	}

	return nil
}

// Runs the C library example
func Example_run() error {
	mg.Deps(Example_build)
	err := sh.RunV("clib/example/example.out")
	if err != nil {
		return err
	}

	return nil
}
