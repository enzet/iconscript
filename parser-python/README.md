# iconscript Python Package

Python parser and converter for the IconScript language.

## Installation

```bash
pip install .
```

## Usage

### Command Line

```bash
iconscript -i $INPUT_FILE -o $OUTPUT_DIRECTORY
```

This will generate SVG files in the `$OUTPUT_DIRECTORY`.

### Python API

```python
from iconscript import iconscript_to_svg
from pathlib import Path

iconscript_file_path: Path = ...
output_directory: Path = ...

iconscript_to_svg(iconscript_file_path, output_directory)
```

## Requirements

- Python >= 3.12
- antlr4-python3-runtime
- svgwrite
- shapely
