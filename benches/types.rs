use criterion::*;
use kaprekars_constants::algo_impls::*;

fn test_algo16_simple(c: &mut Criterion) {
    c.bench_function("test type i16", |b| {
        b.iter(|| algo16::count(black_box(7977)))
    });
}

fn test_algo32_simple(c: &mut Criterion) {
    c.bench_function("test type i32", |b| {
        b.iter(|| algo32::count(black_box(7977)))
    });
}

fn test_algou32_simple(c: &mut Criterion) {
    c.bench_function("test type u32", |b| {
        b.iter(|| algou32::count(black_box(7977)))
    });
}

fn test_algo64_simple(c: &mut Criterion) {
    c.bench_function("test type i64", |b| {
        b.iter(|| algo64::count(black_box(7977)))
    });
}

fn test_algoisize_simple(c: &mut Criterion) {
    c.bench_function("test type isize", |b| {
        b.iter(|| algoisize::count(black_box(7977)))
    });
}

fn test_algo128_simple(c: &mut Criterion) {
    c.bench_function("test type i128", |b| {
        b.iter(|| algo128::count(black_box(7977)))
    });
}

criterion_group!(
    benches,
    test_algo16_simple,
    test_algo32_simple,
    test_algou32_simple,
    test_algo64_simple,
    test_algoisize_simple,
    test_algo128_simple,
);
criterion_main!(benches);
