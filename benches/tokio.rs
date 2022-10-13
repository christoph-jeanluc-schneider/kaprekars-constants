use criterion::*;
use kaprekars_constants::{numbers::gen, tokio::count_all};

fn test_tokio_base(c: &mut Criterion) {
    let chunk_size = 8991;
    let nums = gen();
    c.bench_function("count all tokio (1)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_4(c: &mut Criterion) {
    let chunk_size = 8991 / 3;
    let nums = gen();
    c.bench_function("count all tokio (4)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_8(c: &mut Criterion) {
    let chunk_size = 8991 / 7;
    let nums = gen();
    c.bench_function("count all tokio (8)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_16(c: &mut Criterion) {
    let chunk_size = 8991 / 15;
    let nums = gen();
    c.bench_function("count all tokio (16)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_32(c: &mut Criterion) {
    let chunk_size = 8991 / 31;
    let nums = gen();
    c.bench_function("count all tokio (32)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_128(c: &mut Criterion) {
    let chunk_size = 8991 / 127;
    let nums = gen();
    c.bench_function("count all tokio (128)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x8_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x8_ (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(8)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x12_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x12 (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(12)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x16_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x16 (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(16)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x20_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x20 (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(20)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x24_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x24 (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(24)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x40_64(c: &mut Criterion) {
    let chunk_size = 8991 / 63;
    let nums = gen();
    c.bench_function("count all tokio x40 (64)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(40)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

fn test_tokio_x20_128(c: &mut Criterion) {
    let chunk_size = 8991 / 127;
    let nums = gen();
    c.bench_function("count all tokio x20 (128)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(20)
                .enable_all()
                .build()
                .unwrap(),
        )
        .iter(|| count_all(black_box(nums.to_owned()), chunk_size))
    });
}

criterion_group!(
    benches,
    // test_tokio_base,
    // test_tokio_4,
    // test_tokio_8,
    // test_tokio_16,
    // test_tokio_32,
    // test_tokio_64,
    // test_tokio_128,
    // test_tokio_x8_64,
    // test_tokio_x12_64,
    // test_tokio_x16_64,
    // test_tokio_x20_64,
    // test_tokio_x24_64,
    // test_tokio_x40_64,
    test_tokio_x20_128,
);
criterion_main!(benches);
