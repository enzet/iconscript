package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/enzet/iconscript/grammar/parser"
)

// Parse iconscript using ANTLR.
func parse(commands string) {

	stream := antlr.NewInputStream(commands)
	lexer := parser.NewiconscriptLexer(stream)

	for {
		t := lexer.NextToken()
		if t.GetTokenType() == antlr.TokenEOF {
			break
		}
		fmt.Printf("%s (%q)\n",
			lexer.SymbolicNames[t.GetTokenType()], t.GetText())
	}
}

// Script entry point: use `-i` to specify file name or `-c` to specify string
// commands.
func main() {

	fileName := flag.String("i", "icons.txt", "input file name")
	commands := flag.String("c", "", "input string")
	flag.Parse()

	if *commands == "" {
		content, _ := os.ReadFile(*fileName)
		parse(string(content))
	} else {
		parse(*commands)
	}
}
