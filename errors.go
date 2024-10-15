package libmus

import (
	"errors"
	"fmt"
)

type errWithDetails struct {
	err     error
	details string
}

func errorWithDetails(err error, details string) errWithDetails {
	return errWithDetails{
		err:     err,
		details: details,
	}
}

func (e errWithDetails) Error() string {
	return fmt.Sprintf("%s: %s", e.err, e.details)
}

func (e errWithDetails) Is(err error) bool {
	return errors.Is(e.err, err)
}
