# Tree2

A modern, feature-rich file tree visualization tool written in Rust, similar to the Unix `tree` command but with enhanced capabilities and customization options.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager)

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

## Project Structure

```
tree2/
├── src/
│   ├── app.rs        # Command-line arguments and main execution logic
│   ├── config.rs     # Configuration struct and indentation utilities
│   ├── error.rs      # Custom error types for file operations
│   ├── filter.rs     # Filtering logic for directory entries
│   ├── fmt.rs        # Tree formatting and printing logic
│   └── main.rs       # Program entry point
├── .gitignore        # Git ignore file
├── Cargo.toml        # Project dependencies and metadata
├── Cargo.toml        # Project dependencies and metadata
├── LICENSE           # MIT License
└── README.md         # Project documentation
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Contact

- **Email**: zhongyuan03@outlook.com
- **GitHub**: [pzzzyda](https://github.com/pzzzyda)

## Acknowledgments

- Inspired by the classic Unix `tree` command
- Built with [Rust](https://www.rust-lang.org/) programming language
- Uses [clap](https://github.com/clap-rs/clap) for command-line argument parsing
- Uses [colored](https://github.com/mackwic/colored) for terminal colorization
