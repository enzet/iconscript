package main

import (
	"testing"

	"github.com/antlr4-go/antlr/v4"
	"github.com/stretchr/testify/assert"
)

// checkIcon parses exactly one icon.
func checkIcon(t *testing.T, command string) *Icon {

	parsed, err := parse(antlr.NewInputStream(command))
	icons := parsed.icons

	assert.Equal(t, nil, err, "Error.")
	assert.Equal(t, 1, len(icons), "Unexpected number of icons.")

	icon := icons[0]
	assert.Equal(t, "icon_0", icon.name)
	assert.Equal(t, 1, len(icon.figures), "Unexpected number of figures.")

	return icon
}

// TestLine tests not filled line parsing.
func TestLine(t *testing.T) {

	icon := checkIcon(t, "icon temp = { l 0,0 1,1 }")
	assert.Equal(t, &Line{[]Position{{0, 0}, {1, 1}}, false, 1},
		icon.figures[0])
}

// TestLineFilled tests filled line parsing.
func TestLineFilled(t *testing.T) {

	icon := checkIcon(t, "icon temp = { lf 0,0 1,1 }")
	assert.Equal(t, &Line{[]Position{{0, 0}, {1, 1}}, true, 1},
		icon.figures[0])
}

// TestArc tests arc parsing.
func TestArc(t *testing.T) {

	icon := checkIcon(t, "icon temp = { a 1,1 5 0.1 0.2 }")
	assert.Equal(t, &Arc{Position{1, 1}, 5, 0.1, 0.2, 1}, icon.figures[0])
}

// TestCircle tests circle parsing.
func TestCircle(t *testing.T) {

	icon := checkIcon(t, "icon temp = { c 1,1 5 }")
	assert.Equal(t, &Circle{Position{1, 1}, 5, 1}, icon.figures[0])
}

// TestRectangle tests rectangle parsing.
func TestRectangle(t *testing.T) {

	icon := checkIcon(t, "icon temp = { s 1,1 2,2 }")
	assert.Equal(t, &Rectangle{Position{1, 1}, Position{2, 2}, 1},
		icon.figures[0])
}
