#!/bin/bash
# Generate parsers from `IconScript.g4` grammar file.

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GRAMMAR_FILE="$PROJECT_ROOT/grammar/IconScript.g4"

OUTPUT_DIR="$PROJECT_ROOT/grammar"

if ! command -v antlr4 &> /dev/null && ! command -v antlr &> /dev/null; then
    echo "Error: ANTLR is not installed or not in \`PATH\`." >&2
    exit 1
fi

ANTLR_CMD="antlr4"
if ! command -v antlr4 &> /dev/null; then
    ANTLR_CMD="antlr"
fi

$ANTLR_CMD -Dlanguage=JavaScript -o "$OUTPUT_DIR" "$GRAMMAR_FILE"

echo "Grammars generated into \`$OUTPUT_DIR\`."