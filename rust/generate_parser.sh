#!/bin/bash

# Script to generate Rust parser from ANTLR grammar
# This uses the ANTLR4 Rust tool

set -e

GRAMMAR_FILE="../grammar/IconScript.g4"
OUTPUT_DIR="src/parser"
ANTLR_JAR="antlr4rust.jar"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Check if ANTLR Rust jar exists
if [ ! -f "$ANTLR_JAR" ]; then
    echo "Downloading ANTLR4 Rust target..."
    curl -L -o "$ANTLR_JAR" \
        "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar"

    if [ $? -ne 0 ]; then
        echo "ERROR: Failed to download ANTLR Rust jar."
        echo "Please download manually from:"
        echo "  https://github.com/rrevenantt/antlr4rust/releases"
        exit 1
    fi
fi

# Generate parser
echo "Generating Rust parser from $GRAMMAR_FILE..."
java -jar "$ANTLR_JAR" -Dlanguage=Rust -visitor -o src/grammar "$GRAMMAR_FILE"

# Move generated files to correct location
echo "Moving generated files to $OUTPUT_DIR..."
mv src/grammar/iconscript*.rs "$OUTPUT_DIR/" 2>/dev/null || true

echo "Parser generated successfully!"
echo "Generated files:"
ls -lh "$OUTPUT_DIR"/*.rs
