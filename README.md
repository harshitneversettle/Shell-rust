# Rust Shell

A fully-functional Unix shell implementation written in Rust, featuring command execution, I/O redirection, piping, command history, and tab completion.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

### Core Functionality
- **Built-in Commands**: `echo`, `cd`, `pwd`, `type`, `history`, `exit`, `ls`
- **External Command Execution**: Run any executable in your `$PATH`
- **Command History**: Navigate previous commands using arrow keys (↑/↓)
- **Tab Completion**: Auto-complete built-in commands and executables
- **Quote Handling**: Support for single quotes (`'`), double quotes (`"`), and escape sequences (`\`)

### Advanced Features
- **I/O Redirection**:
  - Standard output: `>` (overwrite), `>>` (append)
  - Standard error: `2>` (overwrite), `2>>` (append)
- **Piping**: Single and multi-stage pipes (`|`)
  - Example: `cat file.txt | grep "pattern" | wc -l`
- **Signal Handling**: Graceful exit with `Ctrl+C`

## Quick Start

### Prerequisites
- Rust 1.70 or higher
- Unix-like operating system (Linux, macOS, WSL)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-shell.git
cd rust-shell

# Build the project
cargo build --release

# Run the shell
cargo run --release
```

## Usage

### Basic Commands
<p align="center">
  <img width="1121" alt="Basic Commands Demo" src="https://github.com/user-attachments/assets/1cfb3e86-05d8-436a-b66a-ed10029e5911" />
</p>

### Advanced Commands
<p align="center">
  <img width="1121" alt="Advanced Commands Demo" src="https://github.com/user-attachments/assets/ea9e8e77-dfff-4e07-bf1c-00cd954583c3" />
</p>
## Project Structure

```
Shell-rust/
├── src/
│   ├── main.rs           # Entry point and main REPL loop
│   ├── parser.rs         # Input parsing with quote/escape handling
│   ├── commands.rs       # Built-in command implementations
│   ├── input.rs          # Terminal input handling and history navigation
│   ├── pipes.rs          # Single and multi-stage pipe execution
│   ├── redirect.rs       # File I/O redirection logic
│   └── auto_complete.rs  # Tab completion for commands
├── Cargo.toml
└── README.md
```

## Technical Details

### Architecture

The shell follows a modular architecture with clear separation of concerns:

1. **Input Module** (`input.rs`): Handles raw terminal input using `crossterm`
2. **Parser Module** (`parser.rs`): Tokenizes input while respecting quotes and escapes
3. **Command Module** (`commands.rs`): Executes built-in commands
4. **Pipe Module** (`pipes.rs`): Manages process piping and stdout/stdin redirection
5. **Redirect Module** (`redirect.rs`): Handles file descriptor redirection

### Key Components

#### Input Parsing
The parser handles complex shell syntax including:
- Single quotes (literal strings, no escaping)
- Double quotes (allows escape sequences like `\"` and `\\`)
- Backslash escaping outside quotes
- Whitespace tokenization

```rust
// Example: echo "hello \"world\"" -> ["echo", "hello \"world\""]
let tokens = parser::parse_input(r#"echo "hello \"world\"""#);
```

### Dependencies

```toml
[dependencies]
crossterm = "0.27"  # Terminal manipulation and raw mode
```

## Built-in Commands

| Command | Description | Example |
|---------|-------------|---------|
| `echo` | Print arguments to stdout | `echo "Hello World"` |
| `cd` | Change directory | `cd /tmp` or `cd ~` |
| `pwd` | Print working directory | `pwd` |
| `type` | Show command type/location | `type echo` |
| `history` | Show command history | `history` or `history 5` |
| `ls` | List directory contents | `ls` |
| `exit` | Exit the shell | `exit` |

## Usage Examples

### Basic Commands
```bash
$ echo "Rust is awesome!"
Rust is awesome!

$ cd /tmp
$ pwd
/tmp
```

### I/O Redirection
```bash
# Redirect stdout to file (overwrite)
$ echo "Hello" > output.txt

# Redirect stdout to file (append)
$ echo "World" >> output.txt

# Redirect stderr to file
$ ls nonexistent 2> errors.txt
```

### Piping
```bash
# Single pipe
$ cat file.txt | grep "pattern"

# Multi-stage pipeline
$ cat file.txt | grep "error" | wc -l

# Pipe with built-in commands
$ echo "test" | grep "test"
```

### Tab Completion
```bash
$ ec<TAB>     # Completes to "echo "
$ pw<TAB>     # Completes to "pwd "
$ gr<TAB>     # Completes to "grep " (if in PATH)
```

### Command History
```bash
$ echo "first command"
$ echo "second command"
$ <UP>        # Shows: echo "second command"
$ <UP>        # Shows: echo "first command"
$ <DOWN>      # Shows: echo "second command"
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Known Limitations

- Unix-only (uses Unix-specific file permission APIs)
- No job control (background processes with `&`)
- No environment variable expansion (e.g., `$HOME`)

## Future Enhancements

- [ ] Environment variable support (`$VAR`, `export`, `unset`)
- [ ] Job control (background processes, `fg`, `bg`, `jobs`)
- [ ] Shell scripting support (if statements, loops, functions))
- [ ] More built-in commands (`alias`, `unalias`, `source`)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built as a learning project to understand Unix shell internals
- Inspired by classic Unix shells (bash, zsh, sh)

## Contact

Harshit Yadav : https://x.com/Harshit_yad4v

Project Link : https://github.com/harshitneversettle/Shell-rust

---
