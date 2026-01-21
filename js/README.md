## Installation

Install dependencies:

```shell
npm install
```

## Development Setup

Generate grammar files:

```shell
npm run generate:grammar
```

Build all components (library, CLI, and web bundles):

```shell
npm run build
```

Build individually:
- `npm run generate:grammar` — generate grammar files,
- `npm run build:lib` — build TypeScript library,
- `npm run build:cli` — build CLI tool,
- `npm run build:parser:min` — build parser bundle for web,
- `npm run build:ui:min` — build UI bundle for web.

## Usage

### Command-Line Interface

```shell
npm run generate $INPUT_ICONSCRIPT_FILE $OUTPUT_DIR
```

### Web Interface

1. Build the web bundles (if not already built):

```shell
npm run build:parser:min
npm run build:ui:min
```

2. Start a local server (e.g., using `live-server`):

```shell
npm install -g live-server
live-server web
```

Or use any other static file server pointing to the `web/` directory.
