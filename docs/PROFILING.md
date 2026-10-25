# Profiling

How to profile QCTidy with [samply](https://github.com/mstange/samply), a
sampling profiler that shows a flame graph in the browser.

## Install samply

```bash
cargo install samply
```

On Linux, samply also needs `kernel.perf_event_paranoid` set to `1` or lower.
If profiling fails with a `perf_event_paranoid` message, lower it with:

```bash
echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid
```

This change is **temporary** (it resets on the next reboot) and is a
system-wide security tweak; restore it to `2` when you are done.

## Build a release binary with symbols

Profiling works best on an optimized build that still keeps function names and
line numbers. Set `CARGO_PROFILE_RELEASE_DEBUG` for the line info and override
the project's `strip = true` with `CARGO_PROFILE_RELEASE_STRIP=none` (otherwise
the binary is stripped and samply only sees raw addresses):

```bash
CARGO_PROFILE_RELEASE_DEBUG=1 CARGO_PROFILE_RELEASE_STRIP=none cargo build --release -p qctidy-cli
```

The `strip = true` comes from `[profile.release]` in the root `Cargo.toml`, so it
must be overridden for profiling.

## Generate a reproducible input

A tiny circuit finishes too fast to profile. Generate a large, deterministic
Qiskit circuit instead:

```bash
cargo run --release -p benchmarks --bin generate_circuit -- -o /tmp/profile_circuit.py
```

The defaults (50 qubits, 1000 time steps, seed 42) produce a circuit of about
34 000 lines that `qctidy check` takes roughly 5 minutes to process. The output
only depends on the seed, so the circuit is reproducible.

Tune the size and seed with the flags:

```bash
cargo run --release -p benchmarks --bin generate_circuit -- \
  --seed 7 --qubits 100 --time-steps 500 -o /tmp/profile_circuit.py
```

Options: `--seed`, `--qubits`, `--time-steps`, `--circuit-name`, and
`-o`/`--output` (writes to standard output when `-o` is omitted).

> The generator builds gate-only circuits (no measurements), because the fixer
> — the usual hot path — does not touch measurements.

## Profile a command

Record the profile to a file, then load it with `samply load`, which serves the
symbols:

```bash
samply record --save-only -o check.json -- target/release/qctidy check -i /tmp/profile_circuit.py
samply load check.json
```

`samply load` opens the browser and supplies symbols by reading the binary's
debug info. Loading `check.json` directly in <https://profiler.firefox.com/>
(drag and drop) does not show symbols: the online profiler cannot read the local
binary, so it only shows raw addresses.

Other commands worth profiling:

```bash
# Metrics (analyzer)
samply record target/release/qctidy analyze -i /tmp/profile_circuit.py --detailed

# Parse/serialize (converter)
samply record target/release/qctidy convert -i /tmp/profile_circuit.py --output-format json
```

## Reading the flame graph

The widest bars are the functions that spend the most CPU time. `check` on the
generated circuit scales worse than linearly with the number of gates, so the
bottleneck is somewhere in the fixer's pattern matching. Look for the widest
frames under `fixer` and `matcher`.
