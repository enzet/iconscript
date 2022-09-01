package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/enzet/iconscript/grammar/parser"
)

func main() {

	var fileName = flag.String("i", "icons.txt", "input file name")
	flag.Parse()

	content, _ := os.ReadFile(*fileName)

	is := antlr.NewInputStream(string(content))
	lexer := parser.NewiconscriptLexer(is)

	for {
		t := lexer.NextToken()
		if t.GetTokenType() == antlr.TokenEOF {
			break
		}
		fmt.Printf("%s (%q)\n",
			lexer.SymbolicNames[t.GetTokenType()], t.GetText())
	}
}
