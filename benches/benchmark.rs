use criterion::*;
use kaprekars_constants::{algo::*, calc::*};

fn benchmark_a(c: &mut Criterion) {
    c.bench_function("a", |b| b.iter(|| CalcResult::from(black_box(7977))));
}

fn benchmark_b(c: &mut Criterion) {
    c.bench_function("b", |b| b.iter(|| count(black_box(7977))));
}

criterion_group!(benches, benchmark_a, benchmark_b);
criterion_main!(benches);
