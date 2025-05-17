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
all: lint test build

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

# Format code
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	$(RUSTFMT) --all

# Run clippy checks with helpful warnings
.PHONY: check
check:
	@echo "Running clippy checks..."
	$(CLIPPY) -- -D warnings \
		-W clippy::pedantic \
		-W clippy::nursery \
		-D clippy::unwrap_used \
		-D clippy::expect_used \
		-W clippy::missing_docs_in_private_items \
		-D clippy::unimplemented \
		-D clippy::todo \
		-W clippy::doc_markdown \
		-A clippy::module_name_repetitions \
		-A clippy::must_use_candidate

# Run very strict clippy checks (good for CI)
.PHONY: check-strict
check-strict:
	@echo "Running strict clippy checks..."
	$(CLIPPY) -- -D warnings \
		-D clippy::pedantic \
		-D clippy::nursery \
		-D clippy::unwrap_used \
		-D clippy::expect_used \
		-W clippy::missing_docs_in_private_items \
		-D clippy::unimplemented \
		-D clippy::todo \
		-D clippy::missing_errors_doc \
		-D clippy::float_cmp \
		-D clippy::doc_markdown \
		-D clippy::module_name_repetitions \
		-D clippy::wildcard_imports \
		-D clippy::if_not_else \
		-D clippy::missing_const_for_fn \
		-D clippy::redundant_closure

# Basic clippy check for regular development
.PHONY: lint
lint:
	@echo "Running basic clippy checks..."
	$(CLIPPY)

# Fix clippy warnings where possible
.PHONY: fix
fix:
	@echo "Fixing clippy warnings where possible..."
	$(CLIPPY) --fix --allow-dirty --allow-staged

# Run examples
.PHONY: run-vim-editor
run-vim-editor:
	@echo "Running vim editor example..."
	$(CARGO) run --example vim_editor --features eframe-demo

.PHONY: run-emacs-editor
run-emacs-editor:
	@echo "Running emacs editor example..."
	$(CARGO) run --example emacs_editor --features eframe-demo

.PHONY: run-minimal
run-minimal:
	@echo "Running minimal example..."
	$(CARGO) run --example minimal --features eframe-demo

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

# Watch for changes and test
.PHONY: watch
watch:
	@echo "Watching for changes..."
	$(CARGO) watch -x test

# Check for outdated dependencies
.PHONY: outdated
outdated:
	@echo "Checking for outdated dependencies..."
	$(CARGO) outdated

# Update dependencies
.PHONY: update
update:
	@echo "Updating dependencies..."
	$(CARGO) update

# Run a benchmark (once we have benchmarks)
.PHONY: bench
bench:
	@echo "Running benchmarks..."
	$(CARGO) bench

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

# Display help information
.PHONY: help
help:
	@echo "Ed-Egui Makefile Commands:"
	@echo ""
	@echo "make              - Build, check and test the project"
	@echo "make build        - Build the project"
	@echo "make build-all    - Build with all features"
	@echo "make release      - Build in release mode"
	@echo "make test         - Run tests"
	@echo "make fmt          - Format code"
	@echo "make check        - Run strict clippy checks"
	@echo "make lint         - Run basic clippy checks"
	@echo "make fix          - Fix clippy warnings where possible"
	@echo "make run-basic    - Run the basic editor example"
	@echo "make run-minimal  - Run the minimal example"
	@echo "make doc          - Generate documentation"
	@echo "make doc-open     - Open documentation in browser"
	@echo "make watch        - Watch for changes and test"
	@echo "make bench        - Run benchmarks (once implemented)"
	@echo "make outdated     - Check for outdated dependencies"
	@echo "make update       - Update dependencies"
	@echo "make clean        - Clean build artifacts"
	@echo "make check-strict  - Run very strict clippy checks"
	@echo "make help         - Show this help message"
