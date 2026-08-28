use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use qctidy::{GraphBuilder, simplifier::simplify};

use crate::random_circuit_generator;

const MAX_ITERATIONS: u32 = 5;

/// Large circuit without obvious simplification opportunities.
///
/// Example:
/// q0: H T S H T S ...
/// q1: T S H T S H ...
/// q2: S H T S H T ...
fn already_simplified(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Already Simplified");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(count);

                        for qubit in 0..count {
                            for index in 0..100 {
                                match (qubit + index) % 3 {
                                    0 => builder.push_h(qubit),
                                    1 => builder.push_t(qubit),
                                    2 => builder.push_s(qubit),
                                    _ => unreachable!(),
                                };
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Narrow circuits with increasing depth.
///
/// Example:
/// q0: H T S H T S ...
/// q1: T S H T S H ...
/// q2: S H T S H T ...
fn deep_narrow_circuit(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Deep Narrow Circuit");

    for depth in [100, 500, 1_000, 2_500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(depth),
            depth,
            |bencher, &depth| {
                bencher.iter_batched(
                    || {
                        let qubits = 4;
                        let mut builder = GraphBuilder::new(qubits);

                        for layer in 0..depth {
                            for qubit in 0..qubits {
                                match (qubit + layer) % 3 {
                                    0 => builder.push_h(qubit),
                                    1 => builder.push_t(qubit),
                                    2 => builder.push_s(qubit),
                                    _ => unreachable!(),
                                };
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Wide circuits with increasing numbers of qubits and constant depth.
///
/// Example:
/// q0: H T S ...
/// q1: T S H ...
/// q2: S H T ...
/// ...
fn wide_shallow_circuit(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Wide Shallow Circuit");

    for qubits in [4, 8, 16, 32, 64, 128, 256].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let depth = 10;
                        let mut builder = GraphBuilder::new(count);

                        for layer in 0..depth {
                            for qubit in 0..count {
                                match (qubit + layer) % 3 {
                                    0 => builder.push_h(qubit),
                                    1 => builder.push_t(qubit),
                                    2 => builder.push_s(qubit),
                                    _ => unreachable!(),
                                };
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Large rows of H gates are cancelled out.
///
/// Example:
/// q0: H H H H H H ...
/// q1: H H H H H H ...
fn cancellation_chain(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Cancellation Chain");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(count);

                        for qubit in 0..count {
                            for _ in 0..100 {
                                builder.push_h(qubit);
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Large rows of alternating H X, Y and Z gates are cancelled out.
///
/// Example:
/// q0: H H H H H H ...
/// q1: X X X X X X ...
/// q2: Y Y Y Y Y Y ...
/// q3: Z Z Z Z Z Z ...
fn alternating_cancellation(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Alternating Cancellation");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(count);

                        for qubit in 0..count {
                            match qubit % 4 {
                                0 => {
                                    for _ in 0..100 {
                                        builder.push_h(qubit);
                                    }
                                }
                                1 => {
                                    for _ in 0..100 {
                                        builder.push_x(qubit);
                                    }
                                }
                                2 => {
                                    for _ in 0..100 {
                                        builder.push_y(qubit);
                                    }
                                }
                                3 => {
                                    for _ in 0..100 {
                                        builder.push_z(qubit);
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Nested symmetric chains of single-qubit gates that can be cancelled only one at a time.
///
/// Example:
/// q0: H X Y Z ... Z Y X H
/// q1: H X Y Z ... Z Y X H
fn nested_cancellation_chain(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Nested Cancellation Chain");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(count);

                        for qubit in 0..count {
                            for gate in [0, 1, 2, 3] {
                                match gate {
                                    0 => builder.push_h(qubit),
                                    1 => builder.push_x(qubit),
                                    2 => builder.push_y(qubit),
                                    3 => builder.push_z(qubit),
                                    _ => unreachable!(),
                                };
                            }

                            for gate in [3, 2, 1, 0] {
                                match gate {
                                    0 => builder.push_h(qubit),
                                    1 => builder.push_x(qubit),
                                    2 => builder.push_y(qubit),
                                    3 => builder.push_z(qubit),
                                    _ => unreachable!(),
                                };
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Reducible cascades of CNOT gates.
fn cnot_cascade(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("CNOT Cascade");

    for qubits in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(count);

                        for _ in 0..10 {
                            for index in 0..(count - 1) {
                                builder.push_cx(index, index + 1).unwrap();
                            }

                            for index in (0..(count - 1)).rev() {
                                builder.push_cx(index, index + 1).unwrap();
                            }
                        }

                        builder.build()
                    },
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

/// Random circuits with varying numbers of qubits and time steps.
fn random_circuit(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Random Circuit");

    for (width, height) in [(5, 4), (10, 8), (25, 16), (50, 32)] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{width}x{height}")),
            &(width, height),
            |bencher, &(width, height)| {
                bencher.iter_batched(
                    || random_circuit_generator::generate(0, width, height),
                    |graph| simplify(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    already_simplified,
    deep_narrow_circuit,
    wide_shallow_circuit,
    cancellation_chain,
    alternating_cancellation,
    nested_cancellation_chain,
    cnot_cascade,
    random_circuit
);
