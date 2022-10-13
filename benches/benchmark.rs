use criterion::*;
use kaprekars_constants::algo::*;

fn test_algo_simple(c: &mut Criterion) {
    c.bench_function("kaprekars constants/algo/simple", |b| b.iter(|| count(black_box(7977))));
}

criterion_group!(benches, test_algo_simple);
criterion_main!(benches);
