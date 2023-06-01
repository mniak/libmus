package clib

import (
	"fmt"
	"path/filepath"

	"github.com/magefile/mage/sh"
	"go.uber.org/multierr"
)

func glob(paths ...string) ([]string, error) {
	var allErrors error
	var result []string
	for _, path := range paths {
		matches, err := filepath.Glob(path)
		multierr.AppendInto(&allErrors, err)
		result = append(result, matches...)
	}
	return result, allErrors
}

func Clear() error {
	paths, err := glob(
		"clib/libmus.a",
		"clib/libmus.h",
		"clib/**/*.out",
	)
	if err != nil {
		return err
	}
	if len(paths) == 0 {
		fmt.Println("The clib/ directory was already cleared")
		return nil
	}
	return sh.RunV("rm", paths...)
}
