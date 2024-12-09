# Rust POSIX Shell Implementation

A minimal POSIX-compliant shell implementation written in Rust. This project implements core shell functionality including command execution, path resolution, quote handling, and built-in commands.

## Features

- Command execution with absolute paths and PATH resolution
- Built-in commands: `cd`, `exit`
- Single and double quote handling
- Space-aware argument parsing
- Error reporting with exit codes
- Standard input/output handling
- Docker support for consistent testing environment

## Requirements

- Rust (latest stable version)
- Docker (for containerized testing)
- Git

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-posix-shell.git
cd rust-posix-shell
```

2. Using the test script (recommended for testing):
```bash
chmod +x test.sh        # Make script executable (one-time setup)
./test.sh              # Builds and runs the container
```

3. Build and run locally:
```bash
cd shell
cargo build
cargo run
```

4. Run in Docker (recommended for consistent testing):
```bash
cd shell
docker build -t rust-shell .
docker run -it rust-shell
```

## Test Cases

The shell supports these operations:

1. Absolute Path Commands:
```bash
$ /bin/ls -l /usr/share
```

2. PATH-based Commands:
```bash
$ ls
$ cat /proc/mounts
```

3. Quote Handling:
```bash
$ /usr/bin/printf "The cat's name is %s.\n" 'Theodore Roosevelt'
$ /bin/echo extra    spaces    will    be    ignored
$ /bin/echo "but    not    if    they're    in    quotes"
```

4. Directory Operations:
```bash
$ cd '/proc/sys'
$ pwd
$ cd '/tmp/"hello"'
$ pwd
```

5. Error Handling:
```bash
$ false
error: command exited with code 1
$ /bin/sh -c 'exit 7'
error: command exited with code 7
```

## Implementation Details

### Core Components

- **Command Parsing**: Handles command tokenization with quote awareness
- **Path Resolution**: Supports both absolute paths and PATH-based command lookup
- **Error Handling**: Comprehensive error reporting for various failure scenarios
- **Built-in Commands**: Implementation of essential shell built-ins

### Technical Specifications

- Maximum command line length: 1000 characters
- Maximum arguments per command: 100
- Proper handling of EOF (Ctrl+D)
- Error reporting on stderr
- No external dependencies

## Project Structure

```
shell/
├── src/
│   └── main.rs          # Main shell implementation
├── Cargo.toml           # Rust package manifest
├── Dockerfile          # Container definition for testing
└── README.md           # Project documentation
```

## Docker Environment

The project uses a Debian-based Rust container for consistent testing:

```dockerfile
FROM rust:1.74-bullseye
WORKDIR /usr/src/shell
COPY . .
RUN rm -f Cargo.lock && cargo build
CMD ["./target/debug/shell"]
```

## Error Messages

The shell provides clear error messages for various scenarios:
- Mismatched quotes: `error: mismatched quotes`
- Command not found: `error: command not found: <command>`
- CD failures: `error: cd failed: <reason>`
- Exit codes: `error: command exited with code <code>`

## Author

Christopher Santos

## License

[Your chosen license]

---
*This project was developed as part of a technical assessment demonstrating shell implementation skills in Rust.*
