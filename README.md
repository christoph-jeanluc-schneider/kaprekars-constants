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

### Avg. time for all 8991 using u32

| method                     | avg       |
| -------------------------- | --------- |
| single thread              | 601.63 ns |
| tokio 12 thr. (1 chunk)    | 578.32 µs |
| tokio 12 thr. (4 chunks)   | 214.01 µs |
| tokio 12 thr. (8 chunks)   | 111.68 µs |
| tokio 12 thr. (16 chunks)  | 123.41 µs |
| tokio 12 thr. (32 chunks)  | 103.30 µs |
| tokio 12 thr. (64 chunks)  | 97.589 µs |
| tokio 12 thr. (128 chunks) | 100.01 µs |
| tokio 8 thr. (64 chunks)   | 107.71 µs |
| tokio 12 thr. (64 chunks)  | 98.865 µs |
| tokio 16 thr. (64 chunks)  | 97.658 µs |
| tokio 20 thr. (64 chunks)  | 97.901 µs |
| tokio 20 thr. (128 chunks) | 99.682 µs |
| tokio 24 thr. (64 chunks)  | 97.518 µs |
| tokio 40 thr. (64 chunks)  | 98.247 µs |
