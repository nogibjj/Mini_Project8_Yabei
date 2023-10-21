# Variables
RUST_TARGET = target/release/Mini8

# Default target
all: python rust

# Python targets
python:
	@echo "Running Python version..."
	python main.py

py_install:
	@echo "Installing Python packages..."
	# Add your python package installation commands here

py_lint:
	@echo "Linting Python code..."
	# Add your python linting commands here

py_test:
	@echo "Testing Python version..."
	python test_main.py

py_format:
	@echo "Formatting Python code..."
	# Add your python formatting commands here

py_deploy:
	@echo "Deploying Python code..."
	# Add your python deployment commands here

# Rust targets
rust: $(RUST_TARGET)
	@echo "Running Rust version..."
	./$(RUST_TARGET)

$(RUST_TARGET): src/main.rs
	cargo build --release

install:
	@echo "Updating Rust..."
	# Add any additional Rust setup commands if necessary

rust-version:
	@echo "Checking Rust versions..."
	rustc --version
	rustup --version

format:
	@echo "Formatting Rust code..."
	cargo fmt

lint:
	@echo "Linting Rust code..."
	cargo clippy

test:
	@echo "Testing Rust version..."
	cargo test

generate_and_push:
	@echo "Generating and pushing logs/artifacts..."
	# Add commands for generating and pushing logs or artifacts

# Clean targets
clean:
	@echo "Cleaning up..."
	rm -rf target/
	rm -rf __pycache__/
	rm -rf mylib/__pycache__/

.PHONY: all python rust clean test py_install py_lint py_test py_format py_deploy install rust-version format lint generate_and_push
