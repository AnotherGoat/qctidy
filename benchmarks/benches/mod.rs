mod performance;

use criterion::criterion_main;

criterion_main!(performance::benches);
