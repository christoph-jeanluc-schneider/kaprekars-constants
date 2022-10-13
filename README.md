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
