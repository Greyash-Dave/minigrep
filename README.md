# minigrep

A lightweight and efficient grep-like command-line utility written in Rust that allows you to search for text patterns in files.

## Features

- **Fast text search** - Built with Rust for optimal performance
- **Multiple search modes**:
  - Standard case-sensitive search (`-d`)
  - Case-insensitive search (`-i`)
  - Count occurrences mode (`-n`)
- **Simple and intuitive interface** - Designed for ease of use
- **Minimal dependencies** - Self-contained with no external requirements

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.58.0 or higher)
- Git

### Direct installation from GitHub

Install directly using Cargo:

```bash
cargo install --git https://github.com/yourusername/minigrep.git
```

This command will:
1. Clone the repository
2. Compile the source code
3. Install the binary to your Cargo bin directory (typically `~/.cargo/bin/`)
4. Make it available in your PATH

### Alternative: Manual installation

If you prefer to examine the code before installing:

```bash
# Clone the repository
git clone https://github.com/yourusername/minigrep.git

# Navigate to the project directory
cd minigrep

# Build and install
cargo install --path .
```

### Verifying installation

After installation, verify that minigrep is properly installed:

```bash
minigrep --version
```

## Usage

### Basic syntax

```
minigrep [MODE] QUERY FILENAME
```

Where:
- `MODE` (optional): Specifies the search mode (`-d`, `-i`, or `-n`)
- `QUERY`: The text pattern to search for
- `FILENAME`: The file to search in

### Search modes

#### Default mode: Case-sensitive search (`-d`)

Finds exact matches for the query string:

```bash
minigrep "Rust" example.txt
# OR explicitly specify default mode
minigrep -d "Rust" example.txt
```

#### Case-insensitive search (`-i`)

Finds matches regardless of case:

```bash
minigrep -i "rust" example.txt
# Will match "Rust", "rust", "RUST", etc.
```

#### Count occurrences mode (`-n`)

Counts how many times the pattern appears instead of printing matches:

```bash
minigrep -n "Rust" example.txt
# Output: Found 5 occurrences of 'Rust'
```

### Examples

Searching for lines containing "error" in a log file:
```bash
minigrep "error" application.log
```

Finding all mentions of "API" regardless of case in a documentation file:
```bash
minigrep -i "api" documentation.md
```

Counting how many times "TODO" appears in a source file:
```bash
minigrep -n "TODO" src/main.rs
```

## Error Handling

minigrep provides clear error messages in the following scenarios:

- Insufficient arguments:
  ```
  Problem parsing arguments: not enough arguments
  ```

- Invalid mode argument:
  ```
  Problem parsing arguments: invalid mode argument
  ```

- File not found:
  ```
  No such file or directory (os error 2)
  ```

## Project Structure

```
minigrep/
├── src/
│   ├── main.rs      # Command-line interface and argument parsing
│   └── lib.rs       # Core functionality and search algorithms
├── Cargo.toml       # Project configuration and dependencies
└── README.md        # Documentation
```

## Development

### Running tests

```bash
cargo test
```

### Building locally

```bash
cargo build --release
```

The compiled binary will be available at `./target/release/minigrep`.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the grep utility and the Rust Programming Language book project
- Built with Rust 🦀
