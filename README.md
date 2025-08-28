# shawty

A very simple webpage screenshot utlity built with Rust.

## Usage

Build and run:

```sh
cargo run
```

## Usage

shawty accepts the following args:

- `--url <URL>`: The URL to process (default: empty)
- `--output <FILE>`: Output file name (default: screenshot.png)
- `--keep <VALUE>`: Optional keep argument (default: empty)
- `--delete <VALUE>`: Optional delete argument (default: empty)

Example:

```sh
cargo run -- --url https://example.com --output result.png
```

## License

MIT
