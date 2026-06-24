use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use qsimplify::{GraphBuilder, simplifier::simplify};

fn cancellation_chain(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cancellation Chain");
    for n_qubits in [1, 2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n_qubits), n_qubits, |b, &n| {
            b.iter_batched(
                || {
                    let mut builder = GraphBuilder::new(n);
                    for q in 0..n {
                        for _ in 0..100 {
                            builder.push_h(q);
                        }
                    }
                    builder.build()
                },
                |graph| simplify(black_box(graph), 3),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn cnot_cascade(c: &mut Criterion) {
    let mut group = c.benchmark_group("CNOT Cascade");
    for n_qubits in [2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n_qubits), n_qubits, |b, &n| {
            b.iter_batched(
                || {
                    let mut builder = GraphBuilder::new(n);
                    for _ in 0..10 {
                        for i in 0..(n - 1) {
                            builder.push_cx(i, i + 1).unwrap();
                        }
                        for i in (0..(n - 1)).rev() {
                            builder.push_cx(i, i + 1).unwrap();
                        }
                    }
                    builder.build()
                },
                |graph| simplify(black_box(graph), 3),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn irreducible_brick_wall(c: &mut Criterion) {
    let mut group = c.benchmark_group("Irreducible Brick Wall");
    for n_qubits in [1, 2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n_qubits), n_qubits, |b, &n| {
            b.iter_batched(
                || {
                    let mut builder = GraphBuilder::new(n);
                    for q in 0..n {
                        for _ in 0..25 {
                            builder.push_h(q);
                            builder.push_t(q);
                            if q < n - 1 {
                                builder.push_cx(q, q + 1).unwrap();
                            }
                            builder.push_s(q);
                        }
                    }
                    builder.build()
                },
                |graph| simplify(black_box(graph), 3),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    cancellation_chain,
    cnot_cascade,
    irreducible_brick_wall
);
criterion_main!(benches);
