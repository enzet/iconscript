# iconscript

iconscript is a language for describing pixel-wise pictograms in the style of the [Röntgen](https://github.com/enzet/Roentgen) project.

The grammar of the language is described in the ANTLR4 `grammar/IconScript.g4` file.

## SVG generation

There are two implementations of iconscript for parsing and generating SVG files.

  *  **Rust**: `cargo install iconscript`. Rust implementation is *faster* and *more reliable*. It uses [`linesweeper`](https://docs.rs/linesweeper/latest/linesweeper/) library for Boolean path operations and SVG optimizations. 
  *  **JavaScript** (TypeScript): `npm install iconscript`. JavaScript implementation uses [Paper.js](http://paperjs.org/) library, that may produce wrong outputs. 

## Syntax

Syntax slightly resembles the syntax of path commands in SVG. Positions on the plane are coded as two floating point number separated by comma: `<x>,<y>` or `+<x>,<y>`. `+` means that the position is relative to the `position`.

If current position is (5, 5), `+1,2` will give (6, 7) and update current position to (6, 7) as well.

### Global context

  * `position` — vector of two floats, current position of the cursor.
  * `width` — float, stroke width.
  * `fill` — Boolean, whether objects should be filled.

### Commands

| **Command** | **Description** |
|---|---|
| `subtract` | Set subtraction mode |
| `fill` | Set fill mode |
| `w <float>` | Set `width` to a value |
| `m <position>` | Set `position` to a value |
| `l [<position>]` | Draw lines between positions |
| `e <position> <float>` | Draw circle specified by center point and radius |
| `r <position> <position>` | Draw rectangle specified by top left and bottom right points |
| `a <position> <float> <float> <float>` | Draw arc specified by center point, radius, and two angles in radians |
### Variables

Variables can be defined with `<variable> = [<command>]` and accessed with `@<variable>`.

### Scopes

Scopes group commands together using `{` and `}`. They can be nested and are used to incapsulate context changes.

### Example

```iconscript
square = fill r +0,0 +2,2
icon glider = {
    m 6,2   @square m +4,4 @square
    m +-8,4 @square m +4,0 @square m +4,0 @square
}
```

This code defines a square (`lf`, filled line — polygon with 5 points). It then reuses `square` variable 5 times to draw a glider.

