# kaprekars-constants

Having fun with [kaprekar's constants](https://en.wikipedia.org/wiki/Kaprekar%27s_routine).

## Benchmarks

```bash
cargo bench
```

### Avg. time for algo using differen types

| type    | avg           |
| ------- | ------------- |
| i16     | 83.858 ns     |
| i32     | 34.743 ns     |
| **u32** | **33.025 ns** |
| i64     | 35.945 ns     |
| isize   | 35.667 ns     |
| i128    | 144.29 ns     |

## Avg. time for all 8991 using u32

| method              | avg       |
| ------------------- | --------- |
| single thread       | 669.21 ns |
| tokio (1 thread)    | 1.1990 ms |
| tokio (4 threads)   | 836.54 µs |
| tokio (8 threads)   | 750.21 µs |
| tokio (16 threads)  | 730.35 µs |
| tokio (32 threads)  | 720.94 µs |
| tokio (64 threads)  | 715.48 µs |
| tokio (128 threads) | 714.77 µs |
