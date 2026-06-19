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

# Try to apply some lint suggestions without breaking anything
fix:
    cargo clippy --workspace --fix

# Apply lint suggestions even if there are unstaged changes
force-fix:
    cargo clippy --workspace --fix --allow-dirty

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
    cargo llvm-cov --workspace --all-features --ignore-filename-regex '(asserter|mother)' --html --open

# Set up Cargo for feature compilation testing
setup-hack:
    cargo install cargo-hack

# Check that all the features compile by themselves (doesn't include all possible combinations)
hack:
    cargo hack check --each-feature

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

# Run the REST API server in debug mode (optionally with selected features)
# Example: just serve "converter-json,presenter-graphviz"
serve +features="":
    cargo run -p qsimplify-server {{ if features != "" { "--no-default-features --features " + features } else { "" } }}

# Run the REST API server in release mode (optionally with selected features)
# Example: just serve-release "converter-json,presenter-graphviz"
serve-release +features="":
    cargo run --release -p qsimplify-server {{ if features != "" { "--no-default-features --features " + features } else { "" } }}

# Install cargo-watch for hot reload
setup-watch:
    cargo install cargo-watch

# Run the REST API server with hot reload (optionally with selected features)
# Example: just watch "converter-json,presenter-graphviz"
watch +features="":
    cargo watch -x "run -p qsimplify-server {{ if features != "" { "--no-default-features --features " + features } else { "" } }}"

# Run the TUI application with all features
tui:
    cargo run -p qsimplify-tui
