package main

import (
	"testing"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/stretchr/testify/assert"
)

// Parse exactly one icon.
func checkIcon(t *testing.T, command string) *Icon {

	parsed, err := parse(antlr.NewInputStream(command))
	icons := parsed.icons

	assert.Equal(t, nil, err, "Error.")
	assert.Equal(t, 1, len(icons), "Unexpected number of icons.")

	icon := icons[0]
	assert.Equal(t, "icon0", icon.name)
	assert.Equal(t, 1, len(icon.figures), "Unexpected number of figures.")

	return icon
}

// Test not filled line parsing.
func TestLine(t *testing.T) {

	icon := checkIcon(t, "{ l 0,0 1,1 }")
	assert.Equal(t, &Line{[]Position{{0, 0}, {1, 1}}, false, 1},
		icon.figures[0])
}

// Test filled line parsing.
func TestLineFilled(t *testing.T) {

	icon := checkIcon(t, "{ lf 0,0 1,1 }")
	assert.Equal(t, &Line{[]Position{{0, 0}, {1, 1}}, true, 1},
		icon.figures[0])
}

// Test arc parsing.
func TestArc(t *testing.T) {

	icon := checkIcon(t, "{ ar 1,1 5 0.1 0.2 }")
	assert.Equal(t, &Arc{Position{1, 1}, 5, 0.1, 0.2, 1}, icon.figures[0])
}

// Test circle parsing.
func TestCircle(t *testing.T) {

	icon := checkIcon(t, "{ c 1,1 5 }")
	assert.Equal(t, &Circle{Position{1, 1}, 5, 1}, icon.figures[0])
}

// Test rectangle parsing.
func TestRectangle(t *testing.T) {

	icon := checkIcon(t, "{ s 1,1 2,2 }")
	assert.Equal(t, &Rectangle{Position{1, 1}, Position{2, 2}, 1},
		icon.figures[0])
}

// Test moving current position.
func TestSetPosition(t *testing.T) {

	parsed, err := parse(antlr.NewInputStream("{ p 1,2 p +2,3 }"))
	assert.Equal(t, nil, err)
	assert.Equal(t, &Position{3, 5}, parsed.currentPosition)
}
