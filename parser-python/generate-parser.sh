#!/bin/bash
# Generate Python parser from `IconScript.g4` grammar file.

set -e

# Get the directory where this script is located.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GRAMMAR_FILE="$PROJECT_ROOT/grammar/IconScript.g4"
OUTPUT_DIR="$SCRIPT_DIR/iconscript/parser"

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

# Create output directory if it doesn't exist.
mkdir -p "$OUTPUT_DIR"

# Remove old generated files (but keep __init__.py if it exists).
if [ -d "$OUTPUT_DIR" ]; then
    find "$OUTPUT_DIR" -type f \( -name "*.py" -o -name "*.interp" -o -name "*.tokens" \) ! -name "__init__.py" -delete
fi

# Generate parser files.
echo "Generating parser from $GRAMMAR_FILE..."
echo "Output directory: $OUTPUT_DIR"

cd "$PROJECT_ROOT"
$ANTLR_CMD -Dlanguage=Python3 -o "$OUTPUT_DIR" "$GRAMMAR_FILE"

# Create __init__.py if it doesn't exist.
if [ ! -f "$OUTPUT_DIR/__init__.py" ]; then
    touch "$OUTPUT_DIR/__init__.py"
fi

echo "Parser generation complete!"
echo "Generated files in: $OUTPUT_DIR"

