# Variables
RUST_TARGET = target/release/Mini8

# Default target
all: python rust

# Python targets
python:
	@echo "Running Python version..."
	python main.py

# Rust targets
rust: $(RUST_TARGET)
	@echo "Running Rust version..."
	./$(RUST_TARGET)

$(RUST_TARGET): src/main.rs
	cargo build --release

# Clean targets
clean:
	@echo "Cleaning up..."
	rm -rf target/
	rm -rf __pycache__/
	rm -rf mylib/__pycache__/

# Test targets
test: test-python test-rust

test-python:
	@echo "Testing Python version..."
	python test_main.py

test-rust:
	@echo "Testing Rust version..."
	cargo test

.PHONY: all python rust clean test test-python test-rust
