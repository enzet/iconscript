# iconscript

iconscript is a pretty simple language for describing simple pixel-wise
pictograms in the style of the [Röntgen](https://github.com/enzet/Roentgen)
project.

The grammar of the language is described in the ANTLR4 `grammar/IconScript.g4`
file.

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

| Context command | Description                 |
|-----------------|-----------------------------|
| `r`             | Set removing mode           |
| `w <float>`     | Set __width__ to a value    |
| `p <position>`  | Set __position__ to a value |

| Action command | Description |
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
