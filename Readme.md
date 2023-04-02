# 分别使用`hmac-blake3`和`hmac-sha256`两种HMAC,比较`blake3`、`sha256`两种哈希算法的计算速度

两种hmac方法都分别实现了并行和串行两种计算方式，并进行benchmark

benchmark包含测试并行实现的速度、串行实现的速度和并行和串行的速度对比

## 如何运行速度测试

```sh
# 该命令会运行所有benchmark
cargo bench
```

## 所有定义的benchmark的列表

- `par_hash_sha256`
- `seq_hash_sha256`
- `par_hash_blake3`
- `seq_hash_blake3`
- `par_and_seq_hash_blake3`
- `par_and_seq_hash_sha256`

## 代码主要使用的第三方crate

- [hmac](https://crates.io/crates/hmac)
- [blake3](https://crates.io/crates/blake3)
- [sha2](https://crates.io/crates/sha2)
- [criterion(benchmark工具)](https://crates.io/crates/criterion)
