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

	icon := parsed[0]
	assert.Equal(t, icon.Name, "icon0")
	assert.Equal(t, len(icon.Figures), 1, "Unexpected number of figures.")

	return icon
}

// Test not filled line parsing.
func TestLine(t *testing.T) {

	icon := CheckIcon(t, "{ l 0,0 1,1 }")
	assert.Equal(t,
		*icon.Figures[0].(*Line),
		Line{[]Position{Position{0, 0}, Position{1, 1}}, false},
	)
}

// Test filled line parsing.
func TestLineFilled(t *testing.T) {

	icon := CheckIcon(t, "{ lf 0,0 1,1 }")
	assert.Equal(t,
		*icon.Figures[0].(*Line),
		Line{[]Position{Position{0, 0}, Position{1, 1}}, true},
	)
}

// Test arc parsing.
func TestArc(t *testing.T) {

	icon := CheckIcon(t, "{ ar 1,1 5 0.1 0.2 }")
	assert.Equal(t, *icon.Figures[0].(*Arc), Arc{Position{1, 1}, 5, 0.1, 0.2})
}

// Test circle parsing.
func TestCircle(t *testing.T) {

	icon := CheckIcon(t, "{ c 1,1 5 }")
	assert.Equal(t, *icon.Figures[0].(*Circle), Circle{Position{1, 1}, 5})
}

// Test rectangle parsing.
func TestRectangle(t *testing.T) {

	icon := CheckIcon(t, "{ s 1,1 2,2 }")
	assert.Equal(t, *icon.Figures[0].(*Rectangle),
		Rectangle{Position{1, 1}, Position{2, 2}})
}
