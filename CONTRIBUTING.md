# Contributing

**IMPORTANT**: before committing or pushing, please enable Git hooks:

```bash
git config core.hooksPath .githooks
```

## Publishing as an npm Package

### Build Process

There are two builds:

1. Library build (`build:lib`) compiles TypeScript source to JavaScript in `dist/`.
2. CLI build (`build:cli`) bundles the CLI tool into a single executable file.

### Publishing

1. Generate grammar: `./scripts/generate-grammar.sh`.
2. Build: `npm run build`.
3. Test the CLI locally: `node dist/cli/generate-svg.js test/main.iconscript output/`.
4. Publish to npm: `npm publish`.

The `prepublishOnly` script will automatically run the build.

### Package Structure

When published, the package includes:
- `dist/` — compiled JavaScript files,
  - `cli/generate-svg.js` — CLI executable (bundled, self-contained),
  - `parser.js` — library entry point,
  - `*.d.ts` — TypeScript type definitions,
- `README.md` — documentation,
- `LICENSE` — license file.

### Installation and Usage

Install globally:

```bash
npm install -g iconscript
```

Use the CLI:

```bash
iconscript input.iconscript output/
```