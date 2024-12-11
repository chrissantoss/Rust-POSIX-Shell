.PHONY: build run test clean check fmt clippy docker-build docker-run docker-test

# Rust commands
build:
	cargo build

run:
	cargo run

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings

# Docker commands
docker-build:
	docker build -t shell-test .

docker-run: docker-build
	docker run -it shell-test

docker-test: docker-build
	docker run --rm shell-test

# Clean up
clean:
	cargo clean
	docker rmi shell-test 2>/dev/null || true

# Show available commands
help:
	@echo "Rust commands:"
	@echo "  make build      - Build the Rust project"
	@echo "  make run        - Run locally"
	@echo "  make test       - Run Rust tests"
	@echo "  make check      - Check for compilation errors"
	@echo "  make fmt        - Check code formatting"
	@echo "  make clippy     - Run Rust linter"
	@echo ""
	@echo "Docker commands:"
	@echo "  make docker-build  - Build Docker image"
	@echo "  make docker-run    - Run in Docker"
	@echo "  make docker-test   - Run tests in Docker"
	@echo ""
	@echo "Other commands:"
	@echo "  make clean      - Clean build artifacts and Docker image" 