package main

import (
	"testing"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/stretchr/testify/assert"
)

// Test line parsing.
func TestLine(t *testing.T) {

	parsed, err := parse(antlr.NewInputStream("{ l 0,0 1,1 }"))

	assert.Equal(t, err, nil, "Error.")
	assert.Equal(t, len(parsed), 1, "Unexpected number of icons.")

	for _, icon := range parsed {
		assert.Equal(t, len(icon.Figures), 1, "Unexpected number of figures.")
	}
}
