# iconscript

iconscript is a pretty simple language for describing simple pixel-wise
pictograms in the style of the [Röntgen](https://github.com/enzet/Roentgen)
project.

The grammar of the language is described in the ANTLR4 `grammar/IconScript.g4`
file.

## SVG generation

There are two implementations of iconscript for parsing and generating SVG
files.
  - __Rust__: `cargo install iconscript`.  Rust implementation is _faster_ and
    _more reliable_. It uses
    [`linesweeper`](https://docs.rs/linesweeper/latest/linesweeper/) library for
    Boolean path operations and SVG optimizations.
  - __JavaScript__ (TypeScript): `npm install iconscript`.  JavaScript
  implementation uses [Paper.js](http://paperjs.org/) library, that may produce
  wrong outputs.

## Syntax

Syntax slightly resembles the syntax of path commands in SVG.

### Global context

  - `width` — float, stroke width.
  - `position` — vector (float, float), current position of the cursor.
  - `fill` -- Boolean, whether objects should be filled.

### Commands

`<vector>` is 2D coordinates in the form `<x>,<y>` or `+<x>,<y>` (`+` means
that the position is relative to the __position__).

| Command                                         | Description               |
| ----------------------------------------------- | ------------------------- |
| `subtract`                                      | Set subtraction mode      |
| `fill`                                          | Set fill mode             |
| `m <point>`                                     | Set `position` to a value |
| `w <width>`                                     | Set `width` to a value    |
| `l [<point>]`                                   | Draw polyline             |
| `r <top left point> <bottom right point>`       | Draw rectangle            |
| `e <center> <radius>`                           | Draw circle               |
| `a <center> <radius> <start angle> <end angle>` | Draw arc                  |

### Variables

Variables can be defined with `<variable> = [<command>]` and accessed with
`@<variable>`.

### Scopes

Scopes group commands together using `{` and `}`. They can be nested and are
used to incapsulate context changes.

### Example

```iconscript
square = {fill r +0,0 +2,2}
icon glider = {
    m 6,2   @square m +4,4 @square
    m +-8,4 @square m +4,0 @square m +4,0 @square
}
```

This code defines a square (filled rectangle). It then reuses `square` variable
5 times to draw a glider.
