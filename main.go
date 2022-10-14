// ANTLR-based parser for iconscript.
// Author: Sergey Vartanov <me@enzet.ru>
package main

import (
	"flag"
	"fmt"
	"log"
	"strconv"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/enzet/iconscript/grammar/parser"
)

// Figure is a simple 2D figure on the surface.
type Figure interface {
	setWidth(width float32)
}

// Line is a polyline through a set of positions.
type Line struct {
	positions []Position
	isFilled  bool
	width     float32
}

func (line *Line) setWidth(width float32) {
	line.width = width
}

// Get string representation of a line.
func (line Line) String() string {

	result := "LINE"

	if line.isFilled {
		result += "_FILLED"
	}
	for _, position := range line.positions {
		result += " " + position.String()
	}
	return result
}

type Rectangle struct {
	start Position
	end   Position
	width float32
}

func (rectangle *Rectangle) setWidth(width float32) {
	rectangle.width = width
}

// Get string representation of a rectangle.
func (rectangle Rectangle) String() string {
	return fmt.Sprintf("RECTANGLE %s %s", rectangle.start, rectangle.end)
}

type Circle struct {
	center Position
	radius float32
	width  float32
}

func (circle *Circle) setWidth(width float32) {
	circle.width = width
}

// Get string representation of a circle.
func (circle Circle) String() string {
	return fmt.Sprintf("CIRCLE %s %f", circle.center, circle.radius)
}

// Arc is a part of a circle.
type Arc struct {
	center     Position
	radius     float32
	startAngle float32 // Start angle in radians.
	endAngle   float32 // End angle in radians.
	width      float32
}

func (arc *Arc) setWidth(width float32) {
	arc.width = width
}

// Get string representation of an arc.
func (arc Arc) String() string {
	return fmt.Sprintf("ARC %s %f %f %f", arc.center, arc.radius,
		arc.startAngle, arc.endAngle)
}

// Icon is a 2-dimensional shape, described by the number of figures, that
// should be united or subtracted.
type Icon struct {
	name    string
	figures []Figure
}

// Get string representation of an icon.
func (icon Icon) String() string {

	result := icon.name + "\n"
	for _, figure := range icon.figures {
		result += fmt.Sprintf("    %s\n", figure)
	}
	return result
}

// Context describes current state of the drawing process.
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

// Position is a 2-dimensional point on the plane.
type Position struct {
	x float32
	y float32
}

// Get string representation of a position.
func (position Position) String() string {
	return fmt.Sprintf("%f,%f", position.x, position.y)
}

// Add other position to the given one and return the combination.
func (position *Position) Add(other *Position) Position {

	position.x += other.x
	position.y += other.y
	return Position{position.x, position.y}
}

// Errors are ignored since it should be checked by ANTLR.
func parseFloat(node string) float32 {

	float, _ := strconv.ParseFloat(node, 32)
	return float32(float)
}

// Read position from its string representation.
func readPosition(context parser.IPositionContext) Position {

	x := parseFloat(context.GetX().GetText())
	y := float32(0.0)
	if context.GetY() != nil {
		y = parseFloat(context.GetY().GetText())
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

// ExitLine constructs line and adds it to the current icon.
func (listener *iconScriptListener) ExitLine(context *parser.LineContext) {

	line := new(Line)
	line.setWidth(listener.context.currentWidth)
	positions := context.AllPosition()
	line.positions = make([]Position, len(positions))
	command := context.GetChild(0).GetPayload().(*antlr.CommonToken).GetText()

	if command == "lf" {
		line.isFilled = true
	}
	for i, position := range positions {
		line.positions[i] = parsePosition(position, listener.context)
	}
	if listener.context.currentIcon != nil {
		listener.context.currentIcon.figures =
			append(listener.context.currentIcon.figures, line)
	} else {
		println("Error: no current icon.")
	}
}

// ExitRectangle constructs a rectangle and adds it to the current icon.
func (listener *iconScriptListener) ExitRectangle(
	context *parser.RectangleContext) {

	rectangle := new(Rectangle)
	rectangle.setWidth(listener.context.currentWidth)
	positions := context.AllPosition()

	rectangle.start = parsePosition(positions[0], listener.context)
	rectangle.end = parsePosition(positions[1], listener.context)

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.figures =
			append(listener.context.currentIcon.figures, rectangle)
	} else {
		println("Error: no current icon.")
	}
}

// ExitCircle constructs acircle and adds it to the current icon.
func (listener *iconScriptListener) ExitCircle(context *parser.CircleContext) {

	circle := new(Circle)
	circle.setWidth(listener.context.currentWidth)

	circle.center = parsePosition(context.Position(), listener.context)
	circle.radius = parseFloat(context.FLOAT().GetText())

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.figures =
			append(listener.context.currentIcon.figures, circle)
	} else {
		println("Error: no current icon.")
	}
}

// ExitArc constructs an arc and adds it to the current icon.
func (listener *iconScriptListener) ExitArc(context *parser.ArcContext) {

	arc := new(Arc)
	arc.setWidth(listener.context.currentWidth)
	floats := context.AllFLOAT()

	arc.center = parsePosition(context.Position(), listener.context)
	arc.radius = parseFloat(floats[0].GetText())
	arc.startAngle = parseFloat(floats[1].GetText())
	arc.endAngle = parseFloat(floats[2].GetText())

	if listener.context.currentIcon != nil {
		listener.context.currentIcon.figures =
			append(listener.context.currentIcon.figures, arc)
	} else {
		println("Error: no current icon.")
	}
}

// ExitSetPosition updates current position.
func (listener *iconScriptListener) ExitSetPosition(
	context *parser.SetPositionContext) {

	parsePosition(context.Position(), listener.context)
}

// ExitsetWidth sets current width.
func (listener *iconScriptListener) ExitsetWidth(
	context *parser.SetWidthContext) {

	listener.context.currentWidth = parseFloat(context.FLOAT().GetText())
}

// ExitName stores icon name.
func (listener *iconScriptListener) ExitName(context *parser.NameContext) {
	listener.context.currentIcon.name = context.IDENTIFIER().GetText()
}

// EnterIcon creates a new icon.
func (listener *iconScriptListener) EnterIcon(_ *parser.IconContext) {
	listener.context.currentIcon = new(Icon)
}

// ExitIcon adds constructed icon to the final set.
func (listener *iconScriptListener) ExitIcon(_ *parser.IconContext) {

	if listener.context.currentIcon.name == "" {
		listener.context.currentIcon.name =
			fmt.Sprintf("icon%d", listener.context.unnamedIconId)
		listener.context.unnamedIconId++
	}
	listener.context.icons =
		append(listener.context.icons, listener.context.currentIcon)
}

// Parse iconscript using ANTLR.
func parse(stream antlr.CharStream) (*Context, error) {

	lexer := parser.NewIconScriptLexer(stream)
	tokenStream := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	p := parser.NewIconScriptParser(tokenStream)

	listener := new(iconScriptListener)
	listener.context = &Context{new(Position), 1.0, 0, false, nil, nil}

	antlr.ParseTreeWalkerDefault.Walk(listener, p.Script())

	return listener.context, nil
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
		parsed, err := parse(stream)
		if err == nil {
			log.Print(parsed)
		} else {
			log.Fatal("Failed to parse file.")
		}
	} else {
		stream := antlr.NewInputStream(*commands)
		parsed, err := parse(stream)
		if err == nil {
			log.Print(parsed)
		} else {
			log.Fatal("Failed to parse file.")
		}
	}
}
