# minigrep

A lightweight, modular command-line text search tool written in Rust, inspired by the classic `grep` utility.

## Features

- **Substring search:** Searches line-by-line through text files and prints matching lines.
<!--- **Case-sensitive & case-insensitive modes:** Supports exact matching by default or case-insensitive search via the `IGNORE_CASE` environment variable.-->
- **Modular architecture:** Separates CLI configuration (`config.rs`), file reading and search execution (`reader.rs`), and entry-point error handling (`main.rs`).
- **Clean error handling:** Outputs errors to `stderr` so standard output (`stdout`) stays clean for piping or file redirection.

## Prerequisites

- [Rust and Cargo](https://rust-lang.org)

## Installation

Clone the repository and build the release binary:

```bash
git clone https://github.com
cd minigrep
cargo build --release
```

The compiled executable will be available at `./target/release/minigrep`.

## Usage

Run `minigrep` by passing a search query and a file path:

```bash
cargo run -- <QUERY> <FILE_PATH>
```

### Examples

**Search using the included sample file (`src/baby.txt`):**

```bash
cargo run -- baby src/baby.txt
```

**Case-insensitive search:**

*Linux / macOS:*
```bash
cargo run -- baby src/baby.txt
```

*Windows (PowerShell):*
```powershell
cargo run -- baby src/baby.txt
```

**Redirect output to a file:**

```bash
cargo run -- baby src/baby.txt > results.txt
```

## Running Tests

Run the test suite with Cargo:

```bash
cargo test
```

## Project Structure

```text
minigrep/
├── src/
│   ├── main.rs     # Entry point and top-level error handling
│   ├── lib.rs      # Library root and module exports
│   ├── config.rs   # CLI argument and environment variable parsing
│   ├── reader.rs   # File reading and line-matching search logic
│   └── baby.txt    # Sample text file for testing searches
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## License

This project is open source and available under the [MIT License](LICENSE).
