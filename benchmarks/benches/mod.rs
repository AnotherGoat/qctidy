mod performance;
mod random_circuit_generator;

use criterion::criterion_main;

criterion_main!(performance::benches);
