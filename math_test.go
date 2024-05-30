package libmus

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestModPositive(t *testing.T) {
	assert.Equal(t, 2, mod(-4, 3))
	assert.Equal(t, 0, mod(-3, 3))
	assert.Equal(t, 1, mod(-2, 3))
	assert.Equal(t, 2, mod(-1, 3))
	assert.Equal(t, 0, mod(0, 3))
	assert.Equal(t, 1, mod(1, 3))
	assert.Equal(t, 2, mod(2, 3))
	assert.Equal(t, 0, mod(3, 3))
	assert.Equal(t, 1, mod(4, 3))
}

func TestModRange(t *testing.T) {
	t.Run("rmod(x, 0, b) is equivalent to mod(x, b+1)", func(t *testing.T) {
		const b = 3
		for n := -10; n <= 10; n++ {
			expected := mod(n, b+1)
			t.Run(fmt.Sprintf("%d mod b+1 is %d", n, expected), func(t *testing.T) {
				value := rmod(n, 0, b)
				assert.Equal(t, expected, value)
			})
		}
	})

	t.Run("manual examples", func(t *testing.T) {
		assert.Equal(t, 1, rmod(-5, 1, 3))
		assert.Equal(t, 2, rmod(-4, 1, 3))
		assert.Equal(t, 3, rmod(-3, 1, 3))
		assert.Equal(t, 1, rmod(-2, 1, 3))
		assert.Equal(t, 2, rmod(-1, 1, 3))
		assert.Equal(t, 3, rmod(0, 1, 3))
		assert.Equal(t, 1, rmod(1, 1, 3))
		assert.Equal(t, 2, rmod(2, 1, 3))
		assert.Equal(t, 3, rmod(3, 1, 3))
		assert.Equal(t, 1, rmod(4, 1, 3))
		assert.Equal(t, 2, rmod(5, 1, 3))
		assert.Equal(t, 3, rmod(6, 1, 3))
	})
}
