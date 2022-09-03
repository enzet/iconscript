package main

import (
	"flag"
	"fmt"
	"log"
	"strconv"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/enzet/iconscript/grammar/parser"
)

// Any 2D figure on the surface.
type Figure interface {
	ToString() string
}

// Polyline through a set of positions.
type Line struct {
	Positions []Position
	IsFilled  bool
}

// Get string representation of a line.
func (line Line) ToString() string {

	result := "LINE"

	if line.IsFilled {
		result += "_FILLED"
	}
	for _, position := range line.Positions {
		result += " " + position.ToString()
	}
	return result
}

// 2-dimensional shape, described by the number of figures, that should be
// united or subtracted.
type Icon struct {
	Name    string
	Figures []Figure
}

// Parser listener, that stores current parser state as well.
type iconScriptListener struct {
	*parser.BaseIconScriptListener

	currentPosition *Position
	unnamedIconId   int

	currentIcon *Icon
	icons       []*Icon
}

// 2-dimensional point on the plane.
type Position struct {
	X float32
	Y float32
}

// Get string representation of a position.
func (position Position) ToString() string {
	return fmt.Sprintf("%f,%f", position.X, position.Y)
}

// Add other position to the given one and return the combination.
func (position *Position) Add(other *Position) Position {

	position.X += other.X
	position.Y += other.Y
	return Position{position.X, position.Y}
}

// Parse position from string representation and update current position.
func parsePosition(context parser.IPositionContext,
	currentPosition *Position) Position {

	x, _ := strconv.ParseFloat(context.GetX().GetText(), 32)
	y := 0.0
	if context.GetY() != nil {
		y, _ = strconv.ParseFloat(context.GetY().GetText(), 32)
	} else {
		println("Error: no Y component.")
	}
	position := Position{float32(x), float32(y)}

	if context.GetRelative() != nil {
		return currentPosition.Add(&position)
	} else {
		currentPosition.X = float32(x)
		currentPosition.Y = float32(y)
		return position
	}
}

// Construct line.
func (listener *iconScriptListener) ExitLine(context *parser.LineContext) {

	line := new(Line)
	line.Positions = make([]Position, len(context.AllPosition()))

	command := context.GetChild(0).GetPayload().(*antlr.CommonToken).GetText()
	if command == "lf" {
		line.IsFilled = true
	}
	for index, position := range context.AllPosition() {
		line.Positions[index] =
			parsePosition(position, listener.currentPosition)
	}
	if listener.currentIcon != nil {
		listener.currentIcon.Figures =
			append(listener.currentIcon.Figures, line)
	} else {
		println("Error: no current icon.")
	}
}

// Store icon name.
func (listener *iconScriptListener) ExitName(context *parser.NameContext) {
	listener.currentIcon.Name = context.IDENTIFIER().GetText()
}

// Create new icon.
func (listener *iconScriptListener) EnterIcon(context *parser.IconContext) {
	listener.currentIcon = new(Icon)
}

// Add constructed icon to the final set.
func (listener *iconScriptListener) ExitIcon(context *parser.IconContext) {
	if listener.currentIcon.Name == "" {
		listener.currentIcon.Name =
			fmt.Sprintf("icon%d", listener.unnamedIconId)
		listener.unnamedIconId++
	}
	listener.icons = append(listener.icons, listener.currentIcon)
}

// Parse iconscript using ANTLR.
func parse(stream antlr.CharStream) {

	lexer := parser.NewIconScriptLexer(stream)
	tokenStream := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	p := parser.NewIconScriptParser(tokenStream)

	listener := new(iconScriptListener)
	listener.currentPosition = new(Position)

	antlr.ParseTreeWalkerDefault.Walk(listener, p.Script())

	for _, icon := range listener.icons {
		println(icon.Name)
		for _, figure := range icon.Figures {
			println("   ", figure.ToString())
		}
	}
}

// Script entry point: use `-i` to specify file name or `-c` to specify string
// commands.
func main() {

	fileName := flag.String("i", "icons.txt", "input file name")
	commands := flag.String("c", "", "input string")
	flag.Parse()

	if *commands == "" {
		stream, err := antlr.NewFileStream(*fileName)
		if err != nil {
			log.Fatal(err)
		}
		parse(stream)
	} else {
		stream := antlr.NewInputStream(*commands)
		parse(stream)
	}
}
