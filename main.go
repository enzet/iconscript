package main

import (
	"flag"
	"fmt"
	"log"

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

type iconScriptListener struct {
	*parser.BaseIconScriptListener
}

func (l *iconScriptListener) ExitAssignment(c *parser.AssignmentContext) {
	println(c.GetLeft().GetText(), ":=", c.GetRight().GetText())
}

// Parse iconscript using ANTLR.
func parse(stream antlr.CharStream) {

	lexer := parser.NewIconScriptLexer(stream)
	tokenStream := antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel)
	p := parser.NewIconScriptParser(tokenStream)

	antlr.ParseTreeWalkerDefault.Walk(&iconScriptListener{}, p.Script())
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
