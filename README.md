# Shawty

A command-line tool for taking selective screenshots of web pages using a headless Chrome browser.

## Features

- Take full-page screenshots of any web page
- Isolate specific elements using CSS selectors
- Remove unwanted elements from screenshots
- Save screenshots in PNG format

## Installation

Make sure you have Rust installed, then build from source:

```bash
git clone https://github.com/vo1x/shawty.git
cd shawty
cargo build --release
```

The binary will be available at `target/release/shawty`.

## Usage

### Basic screenshot
```bash
shawty --url https://example.com
```

### Custom output filename
```bash
shawty --url https://example.com --output my_screenshot.png
```

### Keep only specific elements
```bash
shawty --url https://example.com --keep "header, .main-content, footer"
```

### Remove specific elements
```bash
shawty --url https://example.com --keep ".article" --delete ".ads, .sidebar"
```

## Options

- `-u, --url <URL>` - The URL to screenshot
- `-o, --output <OUTPUT>` - Output filename (default: "screenshot.png")
- `--keep <KEEP>` - CSS selectors for elements to keep (comma-separated)
- `--delete <DELETE>` - CSS selectors for elements to delete (comma-separated)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Requirements

- Rust 1.70+ 
- Chrome/Chromium browser (automatically downloaded by chromiumoxide)