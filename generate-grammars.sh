#!/bin/bash
# Generate parsers from `IconScript.g4` grammar file.

set -e

PROJECT_ROOT="$(pwd)"
GRAMMAR_FILE="$PROJECT_ROOT/grammar/IconScript.g4"

OUTPUT_DIR="$PROJECT_ROOT/grammar"

# Check if ANTLR is installed.
if ! command -v antlr4 &> /dev/null && ! command -v antlr &> /dev/null; then
    echo "Error: ANTLR is not installed or not in PATH." >&2
    echo "Please install ANTLR:" >&2
    echo "  macOS: brew install antlr" >&2
    echo "  Or download from: https://www.antlr.org/download.html" >&2
    exit 1
fi

# Use antlr4 if available, otherwise fall back to antlr.
ANTLR_CMD="antlr4"
if ! command -v antlr4 &> /dev/null; then
    ANTLR_CMD="antlr"
fi

# Check if grammar file exists.
if [ ! -f "$GRAMMAR_FILE" ]; then
    echo "Error: Grammar file not found: $GRAMMAR_FILE" >&2
    exit 1
fi

# Create output directories if they don't exist.
mkdir -p "$OUTPUT_DIR"

# Remove old generated files.
if [ -d "$OUTPUT_DIR" ]; then
    find "$OUTPUT_DIR" -type f \( -name "*.js" -o -name "*.interp" -o -name "*.tokens" \) -delete
fi

cd "$PROJECT_ROOT"
$ANTLR_CMD -Dlanguage=JavaScript -o "$OUTPUT_DIR" "$GRAMMAR_FILE"

echo "Grammars generated into $OUTPUT_DIR"