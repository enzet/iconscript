package main

import (
	"flag"
	"fmt"
	"log"
	"strconv"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/enzet/iconscript/grammar/parser"
)

func printTokens(stream antlr.CharStream) {

	lexer := parser.NewIconScriptLexer(stream)

	for {
		token := lexer.NextToken()
		if token.GetTokenType() == antlr.TokenEOF {
			break
		}
		fmt.Printf("%s (%q)\n",
			lexer.SymbolicNames[token.GetTokenType()], token.GetText())
	}
}

// Any 2D figure on the surface.
type Figure interface {
	ToString() string
}

// Polyline through a set of positions.
type Line struct {
	Positions []Position
}

func (line Line) ToString() string {

	result := "LINE"

	for _, position := range line.Positions {
		result += " " + position.ToString()
	}
	return result
}

type Icon struct {
	Name    string
	Figures []Figure
}

type iconScriptListener struct {
	*parser.BaseIconScriptListener

	currentPosition *Position

	currentIcon *Icon
	icons       []*Icon
}

type Position struct {
	X float32
	Y float32
}

func (position Position) ToString() string {
	return fmt.Sprintf("%f,%f", position.X, position.Y)
}

func (position *Position) Add(other *Position) Position {

	position.X += other.X
	position.Y += other.Y
	return Position{position.X, position.Y}
}

func parsePosition(context parser.IPositionContext,
	currentPosition *Position) Position {

	x, _ := strconv.ParseFloat(context.GetX().GetText(), 32)
	y, _ := strconv.ParseFloat(context.GetY().GetText(), 32)
	position := Position{float32(x), float32(y)}

	if context.GetRelative() != nil {
		return currentPosition.Add(&position)
	} else {
		currentPosition.X = float32(x)
		currentPosition.Y = float32(y)
		return position
	}
}

func (listener *iconScriptListener) ExitLine(context *parser.LineContext) {

	line := new(Line)
	line.Positions = make([]Position, len(context.AllPosition()))

	for index, position := range context.AllPosition() {
		line.Positions[index] =
			parsePosition(position, listener.currentPosition)
	}
	listener.currentIcon.Figures = append(listener.currentIcon.Figures, line)
}

func (listener *iconScriptListener) ExitName(context *parser.NameContext) {
	listener.currentIcon.Name = context.IDENTIFIER().GetText()
}

func (listener *iconScriptListener) EnterIcon(context *parser.IconContext) {
	listener.currentIcon = new(Icon)
}

func (listener *iconScriptListener) ExitIcon(context *parser.IconContext) {
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
