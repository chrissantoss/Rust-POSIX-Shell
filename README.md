# Rust Shell Implementation

A minimal shell implementation written in Rust that supports basic command execution, path resolution, and quote handling.

## Setup

Rust (update to 1.74) and Docker are required to build and run the shell.

1. Create a project structure:
```bash
mkdir -p shell/src
cd shell
```

2. Place the provided files in the shell directory:
- `src/main.rs` - Core shell implementation
- `Dockerfile` - Container definition
- `Makefile` - Makefile for building and running the shell
- `testcase_output.txt` - Sample test outputs
- `Cargo.toml` - Project configuration
- `Cargo.lock` - Project dependencies

## Running the Shell

### 1: Using Makefile
```bash
# Build and run in Docker (recommended for consistent testing)
make docker-run

# Other: Running individual commands:
make docker-build    # Just build the Docker image
make docker-test     # Run tests in Docker
make clean          # Clean up Docker resources

# Other: For local development:
make build          # Build the Rust project
make run            # Run locally
make check          # Check for compilation errors
make fmt            # Check code formatting
make clippy         # Run Rust linter

# To see all available commands:
make help
```

Note: Using Docker is recommended to ensure consistent behavior matching the test cases.

## Features

- Command execution with PATH resolution
- Built-in commands: `cd`, `exit`
- Quote handling (single and double quotes)
- Space-aware argument parsing
- Error handling with exit codes

## Example Usage

```bash
# Basic commands
$ ls
$ pwd

# Absolute paths
$ /bin/ls -l

# Quote handling
$ echo "spaces    preserved    in    quotes"
$ echo 'single quoted text'

# Built-ins
$ cd /some/path
$ exit
```

## Requirements

- Rust (latest stable) - for local builds
- Docker - for containerized testing

## Error Handling

The shell handles various error cases:
- Mismatched quotes
- Command not found
- Directory operation failures
- Command exit codes

See `testcase_output.txt` for some of my comprehensive examples and expected outputs.

