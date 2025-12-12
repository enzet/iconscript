# iconscript

iconscript is a pretty simple language for describing simple pixel-wise
pictograms in the style of the [Röntgen](https://github.com/enzet/Roentgen)
project.

The grammar of the language is described in the ANTLR4 `grammar/IconScript.g4`
file.

## Installation

Install dependencies:

```shell
npm install
```

## Development Setup

Generate grammar files:

```shell
./scripts/generate-grammar.sh
```

Build all components (library, CLI, and web bundles):

```shell
npm run build
```

Build individually:
- `npm run build:lib` — build TypeScript library,
- `npm run build:cli` — build CLI tool,
- `npm run build:parser:min` — build parser bundle for web,
- `npm run build:ui:min` — build UI bundle for web.

## Usage

### Command-Line Interface

```shell
npm run generate $INPUT_ICONSCRIPT_FILE $OUTPUT_DIR
```

### Web Interface

1. Build the web bundles (if not already built):

```shell
npm run build:parser:min
npm run build:ui:min
```

2. Start a local server (e.g., using `live-server`):

```shell
npm install -g live-server
live-server web
```

Or use any other static file server pointing to the `web/` directory.

## Syntax

### Global context

  - __width__ — stroke width.
  - __position__ — current position of the cursor.

### Commands

`<position>` is 2D coordinates in the form `<x>,<y>` or `+<x>,<y>` (`+` means
that the position is relative to the __position__).

| Context command | Description                 |
|-----------------|-----------------------------|
| `r`             | Set removing mode           |
| `w <float>`     | Set __width__ to a value    |
| `p <position>`  | Set __position__ to a value |

| Action command | Description |
|---|---|
| `l [<position>]` | Draw lines between positions |
| `lf [<position>]` | Draw filled lines between positions |
| `c <position> <float>` | Draw circle specified by center point and radius |
| `s <position> <position>` | Draw rectangle specified by top left and bottom right points |
| `a <position> <float> <float> <float>` | Draw arc specified by center point, radius, and two angles in radians |

### Variables

Variables can be defined with `<variable> = [<command>]` and accessed with
`@<variable>`.

### Scopes

Scopes group commands together using `{` and `}`. They can be nested and are used
to incapsulate context changes.

### Example

```iconscript
square = lf +0,0 +2,0 +0,2 +-2,0 +0,-2
icon glider = {
    p 6,2   @square p +4,4 @square
    p +-8,4 @square p +4,0 @square p +4,0 @square
}
```

This code defines a square (`lf`, filled line — polygon with 5 points). It then
reuses `square` variable 5 times to draw a glider.
