# Tree2

A modern, feature-rich file tree visualization tool written in Rust, similar to the Unix `tree` command but with enhanced capabilities and customization options.

## Features

-   **Recursive Directory Traversal**: Visualize directory structures in a clear, hierarchical tree format
-   **Command-Line Customization**: Tailor the output to your needs with various options
-   **Color-Coded Output**: Files and directories are colorized for better readability
-   **Cross-Platform**: Works on Windows, macOS, and Linux
-   **Error Handling**: Gracefully handles file system errors with informative messages

## Installation

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/pzzzyda/tree2.git

# Navigate to the project directory
cd tree2

# Alternatively, if you're already in the project directory
cargo build --release

# The compiled binary will be available at target/release/tree2.exe (Windows) or target/release/tree2 (Unix-like systems)
```

## Usage

```bash
# Basic usage (display tree of current directory)
tree2 .

# Display tree with all files (including hidden ones)
tree2 --all .

# Limit the maximum depth of the tree
tree2 --max-depth 3 .

# Show directories only
tree2 --directories-only .

# Disable colored output
tree2 --no-color .

# Use ASCII-only characters for the tree structure
tree2 --ascii .

# Sort the entries alphabetically
tree2 --sort .

# Combine multiple options
tree2 --all --max-depth 2 --directories-only .
```

## Command-Line Options

```
tree2 [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the directory to visualize

Options:
  -a, --all              Show all files including hidden files
  -m, --max-depth <MAX_DEPTH>
                        Maximum depth to traverse
  -d, --directories-only
                        Show directories only
      --no-color        Disable colored output
      --ascii           Use ASCII-only characters for tree structure
      --sort            Sort entries alphabetically
  -h, --help            Print help information
  -V, --version         Print version information
```

## Project Structure

```
tree2/
├── src/
│   ├── app.rs        # Command-line arguments and main execution logic
│   ├── config.rs     # Configuration struct and indentation utilities
│   ├── error.rs      # Custom error types for file operations
│   ├── fmt.rs        # Tree formatting and printing logic
│   └── main.rs       # Program entry point
├── Cargo.toml        # Project dependencies and metadata
└── Cargo.lock        # Lockfile for dependency versions
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue. If you'd like to contribute code:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Acknowledgments

-   Inspired by the classic Unix `tree` command
-   Built with [Rust](https://www.rust-lang.org/) programming language
-   Uses [clap](https://github.com/clap-rs/clap) for command-line argument parsing
-   Uses [colored](https://github.com/mackwic/colored) for terminal colorization
