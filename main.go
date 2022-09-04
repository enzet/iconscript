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
}

// Polyline through a set of positions.
type Line struct {
	Positions []Position
	IsFilled  bool
}

// Get string representation of a line.
func (line Line) String() string {

	result := "LINE"

	if line.IsFilled {
		result += "_FILLED"
	}
	for _, position := range line.Positions {
		result += " " + position.String()
	}
	return result
}

type Rectangle struct {
	Start Position
	End   Position
}

// Get string representation of a rectangle.
func (rectangle Rectangle) String() string {
	return fmt.Sprintf("RECTANGLE %s %s", rectangle.Start, rectangle.End)
}

type Circle struct {
	Center Position
	Radius float32
}

// Get string representation of a circle.
func (circle Circle) String() string {
	return fmt.Sprintf("CIRCLE %s %f", circle.Center, circle.Radius)
}

// Arc, a part of a circle.
type Arc struct {
	Center     Position
	Radius     float32
	StartAngle float32 // Start angle in radians.
	EndAngle   float32 // End angle in radians.
}

// Get string representation of an arc.
func (arc Arc) String() string {
	return fmt.Sprintf("ARC %s %f %f %f", arc.Center, arc.Radius,
		arc.StartAngle, arc.EndAngle)
}

// 2-dimensional shape, described by the number of figures, that should be
// united or subtracted.
type Icon struct {
	Name    string
	Figures []Figure
}

// Get string representation of an icon.
func (icon Icon) String() string {

	result := icon.Name + "\n"
	for _, figure := range icon.Figures {
		result += fmt.Sprintf("    %s\n", figure)
	}
	return result
}

// Drawing context.
type Context struct {
	currentPosition *Position
	currentWidth    float32
	unnamedIconId   int
	isUnionMode     bool

	currentIcon *Icon
	icons       []*Icon
}

// Parser listener, that stores current parser state as well.
type iconScriptListener struct {
	*parser.BaseIconScriptListener
	context *Context
}

// 2-dimensional point on the plane.
type Position struct {
	X float32
	Y float32
}

// Get string representation of a position.
func (position Position) String() string {
	return fmt.Sprintf("%f,%f", position.X, position.Y)
}

// Add other position to the given one and return the combination.
func (position *Position) Add(other *Position) Position {

	position.X += other.X
	position.Y += other.Y
	return Position{position.X, position.Y}
}

// Errors are ignored since it should be checked by ANTLR.
func ParseFloat(node string) float32 {

	float, _ := strconv.ParseFloat(node, 32)
	return float32(float)
}

// Read position from its string representation.
func readPosition(context parser.IPositionContext) Position {

	x := ParseFloat(context.GetX().GetText())
	y := float32(0.0)
	if context.GetY() != nil {
		y = ParseFloat(context.GetY().GetText())
	} else {
		println("Error: no Y component.")
	}
	return Position{x, y}
}

// Parse position from string representation and update current position.
func parsePosition(positionContext parser.IPositionContext,
	context *Context) Position {

	position := readPosition(positionContext)

	if positionContext.GetRelative() != nil {
		return context.currentPosition.Add(&position)
	} else {
		*context.currentPosition = position
		return position
	}
}

// Construct line and add it to the current icon.
func (listener *iconScriptListener) ExitLine(context *parser.LineContext) {

	line := new(Line)
	positions := context.AllPosition()
	line.Positions = make([]Position, len(positions))
	command := context.GetChild(0).GetPayload().(*antlr.CommonToken).GetText()

	if command == "lf" {
		line.IsFilled = true
	}
	for i, position := range positions {
		line.Positions[i] = parsePosition(position, listener.context)
	}
	if listener.context.currentIcon != nil {
		listener.context.currentIcon.Figures =
			append(listener.context.currentIcon.Figures, line)
	} else {
		println("Error: no current icon.")
	}
}

// Construct rectangle and add it to the current icon.
func (listener *iconScriptListener) ExitRectangle(
	context *parser.RectangleContext) {

	rectangle := new(Rectangle)
	positions := context.AllPosition()

	rectangle.Start = parsePosition(positions[0], listener.context)
	rectangle.End = parsePosition(positions[1], listener.context)

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.Figures =
			append(listener.context.currentIcon.Figures, rectangle)
	} else {
		println("Error: no current icon.")
	}
}

// Construct circle and add it to the current icon.
func (listener *iconScriptListener) ExitCircle(context *parser.CircleContext) {

	circle := new(Circle)

	circle.Center = parsePosition(context.Position(), listener.context)
	circle.Radius = ParseFloat(context.FLOAT().GetText())

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.Figures =
			append(listener.context.currentIcon.Figures, circle)
	} else {
		println("Error: no current icon.")
	}
}

// Construct arc and add it to the current icon.
func (listener *iconScriptListener) ExitArc(context *parser.ArcContext) {

	arc := new(Arc)
	floats := context.AllFLOAT()

	arc.Center = parsePosition(context.Position(), listener.context)
	arc.Radius = ParseFloat(floats[0].GetText())
	arc.StartAngle = ParseFloat(floats[1].GetText())
	arc.EndAngle = ParseFloat(floats[2].GetText())

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.Figures =
			append(listener.context.currentIcon.Figures, arc)
	} else {
		println("Error: no current icon.")
	}
}

// Update current position.
func (listener *iconScriptListener) ExitSetPosition(
	context *parser.SetPositionContext) {

	parsePosition(context.Position(), listener.context)
}

func (listener *iconScriptListener) ExitSetWidth(
	context *parser.SetWidthContext) {

	listener.context.currentWidth = ParseFloat(context.FLOAT().GetText())
}

// Store icon name.
func (listener *iconScriptListener) ExitName(context *parser.NameContext) {
	listener.context.currentIcon.Name = context.IDENTIFIER().GetText()
}

// Create new icon.
func (listener *iconScriptListener) EnterIcon(context *parser.IconContext) {
	listener.context.currentIcon = new(Icon)
}

// Add constructed icon to the final set.
func (listener *iconScriptListener) ExitIcon(context *parser.IconContext) {

	if listener.context.currentIcon.Name == "" {
		listener.context.currentIcon.Name =
			fmt.Sprintf("icon%d", listener.context.unnamedIconId)
		listener.context.unnamedIconId++
	}
	listener.context.icons =
		append(listener.context.icons, listener.context.currentIcon)
}

// Parse iconscript using ANTLR.
func parse(stream antlr.CharStream) ([]*Icon, error) {

	lexer := parser.NewIconScriptLexer(stream)
	tokenStream := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	p := parser.NewIconScriptParser(tokenStream)

	listener := new(iconScriptListener)
	listener.context = &Context{new(Position), 1.0, 0, false, nil, nil}

	antlr.ParseTreeWalkerDefault.Walk(listener, p.Script())

	return listener.context.icons, nil
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
