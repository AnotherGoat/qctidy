# Repository Guidelines and Conventions

## Repository Setup

To use this repository, you'll need the following tools:

| Tool                                                                            | Version  | Purpose                                                 |
| ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------- |
| [Visual Studio Code](https://code.visualstudio.com/) (or any other code editor) | (latest) | Source code editor.                                     |
| [rust-analyzer](https://rust-analyzer.github.io/)                               | (latest) | Rust language server.                                   |
| [Git](https://git-scm.com/)                                                     | (latest) | Version control.                                        |
| [prek](https://prek.j178.dev/)                                                  | 0.3.8    | Git pre-commit hooks.                                   |
| [Rust](https://rust-lang.org/)                                                  | 1.95.0   | Programming language for the core library.              |
| [Python](https://www.python.org/)                                               | 3.12     | Manual Python binding tests.                            |
| [uv](https://docs.astral.sh/uv/)                                                | 0.7.21   | Easy Python virtual environment and package management. |

Note: The listed versions are the ones used for development at the time of writing this document. Older versions may work, but are not guaranteed to.

Run this once to set up the pre-commit hooks:

```shell
prek install
prek prepare-hooks
```

## File Structure

- Add any additional crates inside the `crates/` directory.
- Always create a `mod.rs` file inside each submodule directory. Avoid creating a `.rs` file beside the submodule directory with the same name.
- Inside any module that is not `lib.rs`, never create `pub` modules. Keep their visiblity as `pub(crate)` at most.
- Only create public modules if they are meant to be part of the public API, and declare them in the root `lib.rs` module using `pub mod` and `pub use` as needed.
- Keep the public APIs plain, without requiring the user to use highly nested paths. Use `pub use` to convert nested submodules into plain ones.

## Adding Dependencies

- When adding a new dependency, always use the latest compatible version. Check `crates.io` if unsure.
- Don't add crates for features that are already included in other crates in the workspace, such as `serde` and `thiserror`.
- Add dependency versions to the workspace root, then enable them in individual crates using `workspace = true`.
- Only enable dependency features in the crates that need them.

## Naming Conventions

Follow standard Rust conventions:

- Use `snake_case` for functions, variables and modules.
- Use `PascalCase` for structs, enum variants, types and traits.
- Use `SCREAMING_SNAKE_CASE` for constants.

In addition, use the following naming conventions:

- Try to use concise and descriptive variable names. If both aren't posible at the same time, prefer descriptive names over concise.
- Avoid using abbreviations for variable names. For example: `col -> column`, `pos -> position`.
- The same is true for parameters inside lambdas. Avoid using single letter variable names in them.

## Error handling

- Use `thiserror` to define library errors with descriptive error messages.
- Always suffix error types with `Error`.
- Avoid using `unwrap()` in library code.
- Use `expect()` to check invariants that should be "impossible to reach" in production code.
- Leave error handling to the caller in client implementations. Just returning `Result<T>` is fine most of the time.
- If a function can fail or return nothing, make it return `Result<Option<T>>` to separate success with no results from actual errors.

## Documentation

- Use `///` to create documentation for non-private items. Any item that is preceded by the `pub` keyword should have documentation.
- When documenting functions, always start with a one-line summary, and only add more information if there are important edge cases or invariantes that the caller may need to be aware of.
- Don't document function parameters if their role is obvious.
- Avoid adding `//` comments for details that can be represented with better variable names.
- Use `// Note:` to document invariants and implementation details that aren't part of the public API and cannot be documented just by function or variable names.

## Testing Guidelines

- Unit tests live in `#[cfg(test)]` local modules in the same directory as the original source file, with the `_tests` suffix added to the original module name.
- Integration tests live in a `tests/` directory inside the relevant crate.
- Name tests descriptively, but don't add the `test` prefix to its functions. Take the name of the original function as a reference when naming them.
- No minimum coverage threshold is enforced yet, but all public functions should have at least one test covering the "happy" execution path.

## Commit Style

Use [Conventional Commits](https://www.conventionalcommits.org/) format for any commit written by hand. In particular, use these commit types:

- `build`:
  - Changes the build system configuration (mainly `Cargo.toml` files).
  - Adds or replaces external dependencies.
- `chore`:
  - Updates dependencies (only the version changes).
  - Changes general configuration files not related to other types.
  - Reorganizes or renames files and directories.
- `ci`:
  - Changes the CI/CD configuration.
- `docs`:
  - Updates documentation (`.md` files) and does nothing else.
  - Includes changes to documentation comments in the code.
- `feat`:
  - Introduces new features.
- `fix`:
  - Patches bugs in the codebase.
  - Adds very small changes that should've been part of the previous commit.
- `perf`:
  - Improves performance internally without changing behavior of public APIs.
  - Includes changes to benchmarks or performance tests.
- `refactor`:
  - Improves code quality or maintainability without changing behavior.
- `style`:
  - Changes only the formatting of files, rarely needed due to the pre-commit hooks.
- `test`
  - Adds missing unit or integration tests.
  - Improves existing tests.

In addition, follow these conventions:

- For revert, branch merge and pull request merge commits, just keep the original autogenerated commit message as is.
- If a commit does more than one thing but is self-contained (very common), then keep the commit type that fits most of the changes.
- If a commit includes too many changes, then split it into multiple ones. Try keeping commits atomic in general.
- You may include any important additional details in the commit body, but you'll rarely need to do so if the commits are atomic.
- Always add the scope in parentheses. If a change targets the whole repository, use `global` as the commit scope.
- Usually, crate or module names are good enough to qualify as commit scopes.
- Add any meaningful new features to the [CHANGELOG.md](CHANGELOG.md) file.

## Python bindings testing

To easily manage Python virtual environments and dependencies, use [uv](https://docs.astral.sh/uv/). Then you can manually test the bindings using the workflow described below.

Then install the maturin tool:

```shell
uv tool install maturin
```

Create a local Python virtual environment and install Qiskit in it:

```shell
uv venv
uv pip install qiskit
```

Then build the bindings for your platform, in testing mode:

```shell
uv run maturin develop -m crates/qiskit/Cargo.toml
```

Confirm that the bindings are installed:

```shell
uv pip list
```

Write a `test.py` file with the code you want to test:

```python
from numpy import pi
from qiskit import QuantumCircuit
import qsimplify_qiskit

circuit = QuantumCircuit(3, 1)
circuit.h(0)
circuit.cx(0, 1)
circuit.rx(pi/2, 1)
circuit.p(2*pi, 2)
circuit.measure(2, 0)
circuit.swap(1, 2)

print("Graph:")
print(qsimplify_qiskit.display_graph(circuit))

print("Grid:")
print(qsimplify_qiskit.display_grid(circuit))
```

And run it:

```shell
uv run test.py
```
