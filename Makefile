# Ed-Egui Makefile

# Environment variables
CARGO := cargo
CLIPPY := cargo clippy
RUSTFMT := cargo fmt
TESTS := cargo test
EXAMPLES_DIR := examples
SRC_DIR := src

# Default target
.PHONY: all
all: check test build

# Build the project
.PHONY: build
build:
	@echo "Building ed-egui..."
	$(CARGO) build

# Build with all features
.PHONY: build-all
build-all:
	@echo "Building with all features..."
	$(CARGO) build --all-features

# Build in release mode
.PHONY: release
release:
	@echo "Building release version..."
	$(CARGO) build --release

# Run tests
.PHONY: test
test:
	@echo "Running tests..."
	$(TESTS)

# Run with code coverage
.PHONY: coverage
coverage:
	@echo "Running tests with coverage..."
	CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" RUSTDOCFLAGS="-Cpanic=abort" $(TESTS)
	@echo "For more precise coverage, consider installing grcov or tarpaulin"

# Format code
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	$(RUSTFMT)

# Opinionated Clippy check
.PHONY: check
check:
	@echo "Running Clippy with strict checks..."
	$(CLIPPY) -- -D warnings \
		-D clippy::pedantic \
		-D clippy::nursery \
		-D clippy::unwrap_used \
		-D clippy::expect_used \
		-D clippy::unimplemented \
		-D clippy::todo \
		-D clippy::missing_docs_in_private_items \
		-D clippy::float_cmp \
		-D clippy::integer_division \
		-D clippy::redundant_clone \
		-A clippy::must_use_candidate \
		-A clippy::missing_errors_doc \
		-A clippy::module_name_repetitions

# More permissive Clippy check, good for regular development
.PHONY: lint
lint:
	@echo "Running basic Clippy checks..."
	$(CLIPPY) -- -D warnings

# Fix Clippy warnings where possible
.PHONY: fix
fix:
	@echo "Fixing Clippy warnings where possible..."
	$(CLIPPY) --fix --allow-dirty --allow-staged

# Install cargo tools needed for development
.PHONY: install-tools
install-tools:
	@echo "Installing development tools..."
	$(CARGO) install cargo-watch cargo-update

# Watch for changes and test
.PHONY: watch
watch:
	@echo "Watching for changes..."
	cargo watch -x test

# Run the basic editor example
.PHONY: run-basic
run-basic:
	@echo "Running basic editor example..."
	$(CARGO) run --example basic_editor --features eframe-demo

# Run the minimal example
.PHONY: run-minimal
run-minimal:
	@echo "Running minimal example..."
	$(CARGO) run --example minimal --features eframe-demo

# Check for outdated dependencies
.PHONY: outdated
outdated:
	@echo "Checking for outdated dependencies..."
	cargo outdated

# Generate documentation
.PHONY: doc
doc:
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps

# Open documentation in browser
.PHONY: doc-open
doc-open:
	@echo "Opening documentation..."
	$(CARGO) doc --no-deps --open

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

# Document rules of this Makefile
.PHONY: help
help:
	@echo "Ed-Egui Makefile Commands:"
	@echo ""
	@echo "make                 - Build, check and test the project"
	@echo "make build           - Build the project"
	@echo "make build-all       - Build with all features"
	@echo "make release         - Build in release mode"
	@echo "make test            - Run tests"
	@echo "make coverage        - Run tests with code coverage"
	@echo "make fmt             - Format code"
	@echo "make check           - Run strict Clippy checks"
	@echo "make lint            - Run basic Clippy checks"
	@echo "make fix             - Fix Clippy warnings where possible"
	@echo "make install-tools   - Install development tools"
	@echo "make watch           - Watch for changes and test"
	@echo "make run-basic       - Run the basic editor example"
	@echo "make run-minimal     - Run the minimal example"
	@echo "make outdated        - Check for outdated dependencies"
	@echo "make doc             - Generate documentation"
	@echo "make doc-open        - Open documentation in browser"
	@echo "make clean           - Clean build artifacts"
	@echo "make help            - Show this help message"