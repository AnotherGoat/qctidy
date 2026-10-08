use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use qctidy::{GraphBuilder, fixer::fix};

use crate::random_circuit_generator;

const MAX_ITERATIONS: u32 = 5;

fn already_fixed(criterion: &mut Criterion) {
    // Large circuit without obvious fix opportunities.
    //
    // Example:
    // q0: H T S H T S ...
    // q1: T S H T S H ...
    // q2: S H T S H T ...
    let mut group = criterion.benchmark_group("Already Fixed");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(qubit_count);

                        for qubit in 0..qubit_count {
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
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn deep_narrow_circuit(criterion: &mut Criterion) {
    // Narrow circuits with increasing depth.
    //
    // Example:
    // q0: H T S H T S ...
    // q1: T S H T S H ...
    // q2: S H T S H T ...
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
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn wide_shallow_circuit(criterion: &mut Criterion) {
    // Wide circuits with increasing numbers of qubits and constant depth.
    //
    // Example:
    // q0: H T S ...
    // q1: T S H ...
    // q2: S H T ...
    // ...
    let mut group = criterion.benchmark_group("Wide Shallow Circuit");

    for qubits in [4, 8, 16, 32, 64, 128, 256].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let depth = 10;
                        let mut builder = GraphBuilder::new(qubit_count);

                        for layer in 0..depth {
                            for qubit in 0..qubit_count {
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
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn cancellation_chain(criterion: &mut Criterion) {
    // Large rows of H gates are cancelled out.
    //
    // Example:
    // q0: H H H H H H ...
    // q1: H H H H H H ...
    let mut group = criterion.benchmark_group("Cancellation Chain");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(qubit_count);

                        for qubit in 0..qubit_count {
                            for _ in 0..100 {
                                builder.push_h(qubit);
                            }
                        }

                        builder.build()
                    },
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn alternating_cancellation(criterion: &mut Criterion) {
    // Large rows of alternating H X, Y and Z gates are cancelled out.
    //
    // Example:
    // q0: H H H H H H ...
    // q1: X X X X X X ...
    // q2: Y Y Y Y Y Y ...
    // q3: Z Z Z Z Z Z ...
    let mut group = criterion.benchmark_group("Alternating Cancellation");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(qubit_count);

                        for qubit in 0..qubit_count {
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
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn nested_cancellation_chain(criterion: &mut Criterion) {
    // Nested symmetric chains of single-qubit gates that can be cancelled only one at a time.
    //
    // Example:
    // q0: H X Y Z ... Z Y X H
    // q1: H X Y Z ... Z Y X H
    let mut group = criterion.benchmark_group("Nested Cancellation Chain");

    for qubits in [1, 2, 4, 8, 16, 32].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(qubit_count);

                        for qubit in 0..qubit_count {
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
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn cnot_cascade(criterion: &mut Criterion) {
    // Reducible cascades of CNOT gates.
    let mut group = criterion.benchmark_group("CNOT Cascade");

    for qubits in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(qubits),
            qubits,
            |bencher, &qubit_count| {
                bencher.iter_batched(
                    || {
                        let mut builder = GraphBuilder::new(qubit_count);

                        for _ in 0..10 {
                            for index in 0..(qubit_count - 1) {
                                builder.push_cx(index, index + 1).unwrap();
                            }

                            for index in (0..(qubit_count - 1)).rev() {
                                builder.push_cx(index, index + 1).unwrap();
                            }
                        }

                        builder.build()
                    },
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn random_circuit(criterion: &mut Criterion) {
    // Random circuits with varying numbers of qubits and time steps.
    let mut group = criterion.benchmark_group("Random Circuit");

    for (width, height) in [(5, 4), (10, 8), (25, 16), (50, 32)] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{width}x{height}")),
            &(width, height),
            |bencher, &(width, height)| {
                bencher.iter_batched(
                    || random_circuit_generator::generate(0, width, height),
                    |graph| fix(black_box(graph), MAX_ITERATIONS),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    already_fixed,
    deep_narrow_circuit,
    wide_shallow_circuit,
    cancellation_chain,
    alternating_cancellation,
    nested_cancellation_chain,
    cnot_cascade,
    random_circuit
);
