# 0.4.0

  - Add `--version` option to CLI
    ([#1](https://github.com/enzet/iconscript/issues/1)).
  - Check input file version in CLI and reject incompatible files
    ([#12](https://github.com/enzet/iconscript/issues/12)).
  - Support icon generating from SVG sketches (Rust).

# 0.3.0

  - **Breaking change**: unify syntax
    ([#9](https://github.com/enzet/iconscript/issues/9)).
      - Add `fill` command.
      - Remove `lf` command.
      - Make `r` and `e` respect fill and width scope parameters.

# 0.2.0

  - **Breaking change**: use SVG path commands format
    ([#2](https://github.com/enzet/iconscript/issues/2)).
      - Use `e` for circle (ellipse) instead of `c`.
      - Use `m` for position (move) instead of `p`.
      - Use `r` for rectangle insted of `s` (square).
      - Use `subtract` for subtraction insted of `r` (remove).
  - Fix arc drawing — now it isn't approximated by segmented line
    ([#4](https://github.com/enzet/iconscript/issues/4)).

# 0.1.0

  - Add support for simple commands.
  - Add JavaScript parser and viewer.
