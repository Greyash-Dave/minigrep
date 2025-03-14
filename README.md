# Minigrep

A powerful grep-like command-line utility written in Rust that allows you to search for text patterns within files. This tool was created as a learning project to understand Rust concepts including file I/O, error handling, and command-line argument parsing.

<a href="https://youtu.be/Br-xK8r5UQ0">
  <img src="https://raw.githubusercontent.com/Greyash-Dave/Greyash-Dave/main/images/minigrep/1.PNG" alt="Watch the video">
</a> 

<hr>
  <a href="https://youtu.be/Br-xK8r5UQ0">
    🔗 Watch the video on YouTube
  </a>
<hr>

## Features

### Core Functionality
- **Pattern Searching**: Find lines containing specified text patterns
- **Line-by-line Results**: Display matching lines with line numbers
- **Highlighting**: Colorize matching patterns within results for better visibility
- **Count Mode**: Option to only show the number of matches rather than the actual content

### Search Options
- **Case Sensitivity**: Choose between case-sensitive (default) or case-insensitive searching
- **Inverted Matching**: Find lines that do NOT contain the specified pattern
- **Configurable via Environment**: Set default behavior using environment variables

### Output Customization
- **Colorized Output**: Highlight matches in different colors
- **Color Mode Control**: Configure when colors should be used (always, auto, or never)
- **Highlight Color Selection**: Choose from six different highlight colors

## Building the Project

Since this is a learning project, you'll need to build it locally:

```bash
# Clone the repository (if you've pushed it to GitHub)
git clone https://github.com/Greyash-Dave/minigrep.git

# Or simply navigate to your project directory
cd path/to/minigrep

# Build the project
cargo build

# Run in debug mode
cargo run -- [OPTIONS] PATTERN FILENAME

# Build in release mode for better performance
cargo build --release

# Run the release version
./target/release/minigrep [OPTIONS] PATTERN FILENAME
```

## Usage

The basic syntax for Minigrep is:

```bash
minigrep [OPTIONS] PATTERN FILENAME
```

Where:
- `OPTIONS`: Optional flags to modify search behavior
- `PATTERN`: The text pattern you want to search for
- `FILENAME`: The path to the file you want to search within

### Command-line Options

| Option | Description |
|--------|-------------|
| `-i` | Case-insensitive search (ignore letter case when matching) |
| `-v` | Invert match (show only lines that do NOT contain the pattern) |
| `-n` | Count-only mode (display only the number of matching lines) |
| `--color=MODE` | Control when to use colored output. MODE can be: |
|  | • `always`: Always use colors, even when output is redirected |
|  | • `auto`: Use colors only when outputting directly to a terminal (default) |
|  | • `never`: Never use colors in the output |
| `--highlight=COLOR` | Set the highlight color for matches. COLOR can be one of: |
|  | • `red` (default) |
|  | • `green` |
|  | • `yellow` |
|  | • `blue` |
|  | • `magenta` |
|  | • `cyan` |

### Environment Variables

- `CASE_INSENSITIVE`: When set to any value, makes all searches case-insensitive by default (same as using the `-i` flag)

## Examples

### Basic Searching

Search for the word "function" in a file:
```bash
cargo run -- function main.rs
```

Or if using the built binary:
```bash
./target/release/minigrep function main.rs
```

Output will show each line containing "function" with line numbers and highlighted matches.

### Case-Insensitive Search

Search for "error" ignoring case (will match "Error", "ERROR", etc.):
```bash
cargo run -- -i error log.txt
```

### Finding Lines That Don't Match

Find all lines that don't contain the word "warning":
```bash
cargo run -- -v warning log.txt
```

### Counting Matches

Count how many lines contain "TODO" comments:
```bash
cargo run -- -n "TODO" project.rs
```

### Customizing Output Colors

Search using cyan highlighting instead of the default red:
```bash
cargo run -- --highlight=cyan "important" document.txt
```

### Combining Multiple Options

Case-insensitive search with yellow highlighting, showing only the count:
```bash
cargo run -- -i -n --highlight=yellow "deprecated" legacy.rs
```

### Redirecting Output Without Colors

Force color output off when redirecting to a file:
```bash
./target/release/minigrep --color=never "data" large_file.txt > results.txt
```

## How It Works

Minigrep reads the specified file and processes it line by line. For each line, it:

1. Checks if the line contains the search pattern (considering case-sensitivity settings)
2. For normal search mode: Collects matching lines, their line numbers, and the positions of matches within each line
3. For inverted search: Collects non-matching lines
4. For count-only mode: Simply counts matching or non-matching lines

The output is then formatted according to the specified options, with optional colorization of matches to make them stand out.

## Concepts Learned

This project demonstrates several important Rust concepts:

- **Error Handling**: Using Result types to handle potential errors
- **Modules and Organization**: Separating code into library and binary crates
- **Command-Line Parsing**: Processing user input from the command line
- **File I/O**: Reading from files and handling related errors
- **Enums and Pattern Matching**: Using Rust's powerful enum system for options
- **Testing**: Writing unit tests to ensure code functionality
- **String Manipulation**: Working with string and text processing
- **Environment Variables**: Integrating with system environment
- **ANSI Terminal Codes**: Implementing colorized terminal output

## Performance Considerations

- Minigrep loads the entire file into memory, so extremely large files might cause memory issues
- The search algorithm uses Rust's standard string matching, which is efficient for most use cases
- Color processing adds minimal overhead and can be disabled for maximum performance

## Project Structure

- `src/lib.rs`: Contains the core functionality and search algorithms
- `src/main.rs`: Handles command-line argument parsing and program execution

## Future Improvements

Some potential enhancements if you continue working on this project:

- Add support for regular expressions
- Implement recursive directory search
- Add context lines (showing lines before and after matches)
- Support for reading from standard input
- Implement multi-threading for large files
