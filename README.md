# iconscript

Paperscript parser for icon shape drawing.

This is a part of [Map Machine](https://github.com/enzet/map-machine) project.

## Run

```shell
npm install paper live-server
live-server
```

## Syntax

### Variables

Variable can be defined with `<variable> = <value>` and accessed with
`@<variable>`.

### Global state

  - __width__
  - __current position__

### Commands

| Command | Description |
|---|---|
| `a` | Set adding mode (default mode) |
| `r` | Set removing mode |
| `w <float>` | Set __width__ to a value |
| `p <position>` | Set __current position__ to a value |
| `l [<position>]` | Draw line of `width` |
| `c <position> <radius>` | |
| `s <position> <position>` | Draw rectangle specified by top left and bottom right points |
