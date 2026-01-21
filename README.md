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

  - `width` — stroke width.
  - `position` — current position of the cursor.

### Commands

`<vector>` is 2D coordinates in the form `<x>,<y>` or `+<x>,<y>` (`+` means
that the position is relative to the __position__).

| Command                              | Description                                                                       |
| ------------------------------------ | --------------------------------------------------------------------------------- |
| `subtract`                           | Set subtraction mode                                                              |
| `w <float>`                          | Set `width` to a value                                                            |
| `m <vector>`                         | Set `position` to a value                                                         |
| `l [<vector>]`                       | Draw lines between positions                                                      |
| `lf [<vector>]`                      | Draw filled lines between positions                                               |
| `e <vector> <float>`                 | Draw circle specified by center point and radius                                  |
| `r <vector> <vector>`                | Draw rectangle specified by top left and bottom right points                      |
| `a <vector> <float> <float> <float>` | Draw arc specified by center point, radius, start angle, and end angle in radians |

### Variables

Variables can be defined with `<variable> = [<command>]` and accessed with
`@<variable>`.

### Scopes

Scopes group commands together using `{` and `}`. They can be nested and are
used to incapsulate context changes.

### Example

```iconscript
square = lf +0,0 +2,0 +0,2 +-2,0 +0,-2
icon glider = {
    m 6,2   @square m +4,4 @square
    m +-8,4 @square m +4,0 @square m +4,0 @square
}
```

This code defines a square (`lf`, filled line — polygon with 5 points). It then
reuses `square` variable 5 times to draw a glider.
