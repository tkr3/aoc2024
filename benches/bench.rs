use criterion::{criterion_group, criterion_main};

// To run individual benchmarks use:
// $ cargo bench --bench bench -- <name>
// where <name> can be like: day_07, 07, 07/1, 7/2

aoc2024::benches!(day_01, day_02, day_03, day_04, day_05, day_06, day_07);

criterion_main!(benchmarks);
