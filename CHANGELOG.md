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
