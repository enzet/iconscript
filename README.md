# iconscript

Paperscript parser for icon shape drawing.

This is a part of [Map Machine](https://github.com/enzet/map-machine) project.

This project consists of several parts:
  - iconscript language grammar, written in ANTLR4,
  - web interface for interactively creating `*.iconscript` files,
  - SVG generator `iconscript_to_svg.py`, that converts drawing commands to SVG
    files,
  - iconscript parser (Go), that converts `*.iconscript` files to drawing
    commands.

```
web interface -> iconscript
iconscript -main.go-> drawing commands
```

## Web interface

Install:

```shell
npm install paper paperjs-offset
npm install -g live-server
```

Run:

```shell
live-server
```

## Python part



## Go part

Requires: Go 1.22, ANTLR 4.13.2.

Install:

```shell
cd parser-go
cd ../grammar
antlr -Dlanguage=Go -o ../parser-go/parser IconScript.g4
cd ../parser-go
go build
```

You may also test installation with `go test`.

```shell
cd parser-go
go run main.go -i <file path>
```

## Syntax

### Global context

  - __width__ — stroke width.
  - __position__ — current position of the cursor.

### Commands

`<positon>` is 2D coordinates in the form `<x>,<y>` or `+<x>,<y>` (`+` means,
that the position is relative to the __position__).

| Command | Description |
|---|---|
| `a` | Set adding mode (default mode) |
| `r` | Set removing mode |
| `w <float>` | Set __width__ to a value |
| `p <position>` | Set __position__ to a value |
| `l [<position>]` | Draw lines between positions |
| `c <position> <float>` | Draw circle specified by center point and radius |
| `s <position> <position>` | Draw rectangle specified by top left and bottom right points |
| `ar <position> <float> <float> <float>` | Draw arc specified by center point, radius, and two angles in radians |

### Variables

Variable can be defined with `<variable> = [<command>]` and accessed with
`@<variable>`.

### Example

```iconscript
cube = lf +0,0 +2,0 +0,2 +-2,0 +0,-2
{
    %glider
    p 6,2   @cube p +4,4 @cube
    p +-8,4 @cube p +4,0 @cube p +4,0 @cube
}
```
