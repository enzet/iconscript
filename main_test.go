package main

import (
	"testing"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/stretchr/testify/assert"
)

// Parse exactly one icon.
func CheckIcon(t *testing.T, command string) *Icon {

	parsed, err := parse(antlr.NewInputStream(command))

	assert.Equal(t, err, nil, "Error.")
	assert.Equal(t, len(parsed), 1, "Unexpected number of icons.")

	return parsed[0]
}

// Test line parsing.
func CheckLine(t *testing.T, command string, isFilled bool) {

	icon := CheckIcon(t, command)
	assert.Equal(t, len(icon.Figures), 1, "Unexpected number of figures.")

	assert.Equal(t,
		*icon.Figures[0].(*Line),
		Line{[]Position{Position{0, 0}, Position{1, 1}}, isFilled},
	)
}

func TestLine(t *testing.T) {
	CheckLine(t, "{ l 0,0 1,1 }", false)
}

func TestLineFilled(t *testing.T) {
	CheckLine(t, "{ lf 0,0 1,1 }", true)
}

func TestArc(t *testing.T) {
	icon := CheckIcon(t, "{ ar 1,1 5 0.1 0.2 }")
	assert.Equal(t, len(icon.Figures), 1, "Unexpected number of figures.")
	assert.Equal(t, *icon.Figures[0].(*Arc), Arc{Position{1, 1}, 5, 0.1, 0.2})
}
