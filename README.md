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

```
<variable> = <value>
```

### Commands

| Command | Description |
|---|---|
| `l [<position>]` | Draw line of `width` |
| `c <position> <radius>` | |
| `w <float>` | Set __width__ to a value |
| `p <position>` | Set __current position__ to a value |
| `a` | Set adding mode |
| `r` | Set removing mode |
