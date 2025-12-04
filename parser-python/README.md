# iconscript Python Package

Python parser and converter for the IconScript language.

## Installation

```bash
pip install iconscript
```

## Usage

### Command Line

```bash
iconscript input.iconscript
```

This will generate SVG files in the `icons/` directory.

### Python API

```python
from iconscript import iconscript_to_svg
from pathlib import Path

iconscript_to_svg(Path("input.iconscript"))
```

## Requirements

- Python >= 3.12
- antlr4-python3-runtime
- svgwrite
- shapely

