# iconscript

iconscript is a language for describing simple pixel-wise pictograms.

The language is created for the [Röntgen](https://github.com/enzet/Roentgen)
project to automate pictogram generation.

This project consists of several parts:
  - iconscript language grammar, written in ANTLR4,
  - web interface for interactively creating `*.iconscript` files.

```
web interface -> iconscript files
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

## ANTLR4 parser generation

```shell
./generate-grammars.sh
```

## Syntax

### Global context

  - __width__ — stroke width.
  - __position__ — current position of the cursor.

### Commands

`<position>` is 2D coordinates in the form `<x>,<y>` or `+<x>,<y>` (`+` means
that the position is relative to the __position__).

| Command | Description |
|---|---|
| `r` | Set removing mode |
| `w <float>` | Set __width__ to a value |
| `p <position>` | Set __position__ to a value |
| `l [<position>]` | Draw lines between positions |
| `lf [<position>]` | Draw filled lines between positions |
| `c <position> <float>` | Draw circle specified by center point and radius |
| `s <position> <position>` | Draw rectangle specified by top left and bottom right points |
| `a <position> <float> <float> <float>` | Draw arc specified by center point, radius, and two angles in radians |

### Variables

Variables can be defined with `<variable> = [<command>]` and accessed with
`@<variable>`.

### Example

```iconscript
cube = lf +0,0 +2,0 +0,2 +-2,0 +0,-2
icon glider = {
    p 6,2   @cube p +4,4 @cube
    p +-8,4 @cube p +4,0 @cube p +4,0 @cube
}
```
