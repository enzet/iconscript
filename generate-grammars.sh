#!/bin/bash
# Generate parsers from `IconScript.g4` grammar file.

set -e

PROJECT_ROOT="$(pwd)"
GRAMMAR_FILE="$PROJECT_ROOT/grammar/IconScript.g4"

PYTHON_OUTPUT_DIR="$PROJECT_ROOT/parser-python/iconscript/parser"
GO_OUTPUT_DIR="$PROJECT_ROOT/parser-go/parser"

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
mkdir -p "$PYTHON_OUTPUT_DIR"
mkdir -p "$GO_OUTPUT_DIR"

# Remove old generated files (but keep __init__.py if it exists).
if [ -d "$PYTHON_OUTPUT_DIR" ]; then
    find "$PYTHON_OUTPUT_DIR" -type f \( -name "*.py" -o -name "*.interp" -o -name "*.tokens" \) ! -name "__init__.py" -delete
fi
if [ -d "$GO_OUTPUT_DIR" ]; then
    find "$GO_OUTPUT_DIR" -type f \( -name "*.go" -o -name "*.interp" -o -name "*.tokens" \) -delete
fi

cd "$PROJECT_ROOT"
$ANTLR_CMD -Dlanguage=Python3 -o "$PYTHON_OUTPUT_DIR" "$GRAMMAR_FILE"
$ANTLR_CMD -Dlanguage=Go -o "$GO_OUTPUT_DIR" "$GRAMMAR_FILE"

# Create __init__.py if it doesn't exist for Python.
if [ ! -f "$PYTHON_OUTPUT_DIR/__init__.py" ]; then
    touch "$PYTHON_OUTPUT_DIR/__init__.py"
fi

echo "Grammars generated:"
echo "  - Python: $PYTHON_OUTPUT_DIR"
echo "  - Go: $GO_OUTPUT_DIR"