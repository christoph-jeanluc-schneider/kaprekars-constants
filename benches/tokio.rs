use criterion::*;
use kaprekars_constants::{numbers::gen, tokio::count_all};

fn test_tokio_base(c: &mut Criterion) {
    let chunk_size = 8991;
    let nums = gen();
    c.bench_function("count all tokio (1)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_4(c: &mut Criterion) {
    let chunk_size = 8991 / 3;
    let nums = gen();
    c.bench_function("count all tokio (4)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_8(c: &mut Criterion) {
    let chunk_size = 8991 / 7;
    let nums = gen();
    c.bench_function("count all tokio (8)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_16(c: &mut Criterion) {
    let chunk_size = 8991 / 15;
    let nums = gen();
    c.bench_function("count all tokio (16)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_32(c: &mut Criterion) {
    let chunk_size = 8991 / 31;
    let nums = gen();
    c.bench_function("count all tokio (32)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio (64)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

fn test_tokio_128(c: &mut Criterion) {
    let chunk_size = 8991 / 127;
    let nums = gen();
    c.bench_function("count all tokio (128)", |b| {
        b.iter(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { count_all(black_box(nums.to_owned()), chunk_size).await });
        })
    });
}

criterion_group!(
    benches,
    test_tokio_base,
    test_tokio_4,
    test_tokio_8,
    test_tokio_16,
    test_tokio_32,
    test_tokio_64,
    test_tokio_128,
);
criterion_main!(benches);
