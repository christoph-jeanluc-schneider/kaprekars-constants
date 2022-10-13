use criterion::*;
use kaprekars_constants::{algo::*, numbers::gen};

fn test_simple(c: &mut Criterion) {
    let nums = gen();    
    c.bench_function("count all", |b| {
        b.iter(|| {
            black_box(nums.to_owned()).into_iter().map(|n| (n, count(n)))
        })
    });
}

criterion_group!(
    benches,
    test_simple,
);
criterion_main!(benches);
