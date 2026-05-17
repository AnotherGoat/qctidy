# Justfile for QSimplify
#
# Run `just` or `just --list` to see all available recipes

# Show list of available recipes (default)
default:
    @just --list

# Install and prepare pre-commit hooks
setup:
    prek install
    prek prepare-hooks

# Manually run pre-commit hooks on all files
pre-commit:
    prek run --all-files --show-diff-on-failure

# Format all Rust code in the project
format:
    cargo fmt

# Lint all crates in the workspace
lint:
    cargo clippy --workspace

# Check that all crates compile without errors
check:
    cargo check --workspace

# Create and open documentation for all the crates
document:
    cargo doc --workspace --no-deps --open

# Build all crates for development (debug mode)
build:
    cargo build --workspace

# Build a specific crate (optionally with selected features)
# Example: just build-crate converter "presenter,codegen"
build-crate crate +features="":
    cargo build -p qsimplify-{{crate}} {{ if features != "" { "--features " + features } else { "" } }}

# Build all crates for production (optimized release mode)
build-release:
    cargo build --release --workspace

# Build a specific crate in release mode (optionally with selected features)
# Example: just build-crate-release converter "presenter,codegen"
build-crate-release crate +features="":
    cargo build --release -p qsimplify-{{crate}} {{ if features != "" { "--features " + features } else { "" } }}

# Run all tests in all crates
test:
    cargo test --workspace --no-fail-fast

# Run all tests with backtrace enabled for debugging
test-backtrace:
    RUST_BACKTRACE=1 cargo test --workspace

# Run tests for a specific crate
# Example: just test-crate converter
test-crate crate:
    cargo test -p qsimplify-{{crate}}

# Set up Cargo for coverage reporting
setup-coverage:
    cargo install cargo-llvm-cov

# Generate code coverage report for all crates
coverage:
    cargo llvm-cov --workspace --all-features --ignore-filename-regex '(asserter)' --html --open

# Set up Python environment for testing bindings
setup-python:
    uv tool install maturin
    uv sync

# Build and install qiskit bindings package in development mode with default features
qiskit:
    uv run maturin develop -m crates/qiskit/Cargo.toml

# Build and install qiskit bindings package in development mode with selected features only
# Example: just qiskit-features converter,codegen
qiskit-features features:
    uv run maturin develop -m crates/qiskit/Cargo.toml --no-default-features --features {{features}}

# Build qiskit bindings package as a release wheel (for distribution)
qiskit-release:
    uv run maturin build -m crates/qiskit/Cargo.toml --release

# Build qiskit bindings release wheel with selected features only
# Example: just qiskit-release-features converter,codegen
qiskit-release-features features:
    uv run maturin build -m crates/qiskit/Cargo.toml --release --no-default-features --features {{features}}
