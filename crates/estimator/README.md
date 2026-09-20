# `qctidy-estimator`

The `qctidy-estimator` module is a native Rust library that provides advanced tools to predict the **physical execution time** and the **financial cost in US dollars (USD)** of quantum circuits (`Graph`) on real quantum hardware providers, specifically **AWS Braket** and **IBM Quantum**.

This module was designed as a direct, optimized replacement (port) of Python scripts, bringing all data collection and heuristic simulation into Rust's safe and concurrent ecosystem.

---

## Main Features

- **Native Scraper (Web + API)**: Automatically and asynchronously extracts current prices from the internet. It uses the AWS Price List API for Braket and parses the IBM Quantum pricing catalog using `reqwest` and `scraper`.
- **Smart Cache**: Stores prices in a local file (`pricing_cache.json`) valid for 24 hours to avoid saturating the network and dramatically speed up the calculation on subsequent runs.
- **Heuristic Time Engine (DAG)**: Uses a **Critical Path** algorithm over the circuit's Directed Acyclic Graph to estimate the time, accurately computing the degree of parallelism.
- **Financial Calculator**: Cross-references the DAG's heuristic time with the scraper data to generate exact quotes according to each provider's billing model.

---

## How does the time estimation work?

Unlike a simple gate count, `time_estimator` simulates the physical clock of each qubit independently while respecting synchronization:

1. **Heuristic Weights (`BackendProfile`)**: Each gate is assigned an abstract time "weight". For example, 1-qubit gates cost `1.0`, and 2-qubit gates (e.g. CNOT) cost `15.0` due to their complexity on microwave or trapped-ion hardware.
2. **Per-Qubit Clock**: We keep one clock per qubit.
3. **Synchronization (Critical Path)**: We iterate the `Graph` topologically (by columns). When a gate involves multiple qubits, it *cannot run until all the involved qubits are free*. The start time is the `maximum` of those qubits' clocks.
4. **Critical Path**: The total time of a single execution (one "shot") is the maximum value reached by any clock when the circuit finishes.
5. **Total Time**: We multiply this critical path by the number of `shots`, adding the physical QPU reset delay.

---

## Billing Models

The module abstracts the differences in how providers charge for quantum access:

- **IBM Quantum**: Billing by **pure QPU usage time**. The module converts the abstract heuristic time into real milliseconds/seconds using a base conversion factor (`base_time_ns`) and multiplies it by the plan's cost per second (e.g. *Pay-As-You-Go*).
- **AWS Braket**: **Flat** billing. It ignores the critical-path execution time and charges a base fee per *Task* plus a fee per *Shot*, which varies with the selected third-party hardware (IonQ, Rigetti, OQC, etc).

---

## Module Structure

- `src/models.rs`: Data structures, `BackendProfile`, and schemas for the cost reports.
- `src/scraper.rs`: Asynchronous logic to connect to the AWS and IBM pricing APIs and manage the cache.
- `src/time_estimator.rs`: Mathematical algorithm for the DAG and Critical Path.
- `src/calculator.rs`: Financial data cross-referencing logic that produces the final USD quotes.

---

## Usage Example (API)

```rust
use qctidy_estimator::{BackendProfile, estimate_execution_time, get_pricing_data, calculate_costs};
use qctidy::Graph;

// 1. Get the graph (circuit)
// let graph = ...

// 2. Define the hardware profile and simulations
let profile = BackendProfile::default();
let shots = 1000;

// 3. Simulate the physical execution time
let heuristic_time = estimate_execution_time(&graph, shots, &profile);

// 4. Fetch prices from the internet asynchronously (false = use the cache if available)
let pricing_data = get_pricing_data(false).await;

// 5. Get financial quotes in USD
let costs = calculate_costs(&pricing_data, heuristic_time, shots, profile.base_time_ns);

println!("Estimated costs: {:#?}", costs);
```
