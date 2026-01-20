# iconscript implementation in Rust

- Parses iconscript files using ANTLR grammar.
- Generates SVG files using the `linesweeper` library for Boolean path
  operations.

## Building

```bash
cd rust
./generate_parser.sh
cargo build --release
```

The binary will be available at `target/release/iconscript`.

## Usage

```shell
./target/release/iconscript $OPTIONS $ICONSCRIPT_FILE
```

| Option               | Description                                        |
| -------------------- | -------------------------------------------------- |
| `-o`, `--output`     | Output directory for SVG files (default: `output`) |
| `-s`, `--sketch`     | Output raw paths without combining                 |
| `--no-rounding`      | Disable coordinate rounding                        |
| `--no-deduplication` | Disable duplicate point removal                    |
| `--no-collinear`     | Disable collinear point simplification             |

## Testing

Run tests:

```shell
cargo test
```

Try to run on test file:

```shell
cargo run -- ../test/main.iconscript -o test-output/
```
