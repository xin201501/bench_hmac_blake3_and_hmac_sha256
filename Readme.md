# 分别使用`hmac-blake3`和`hmac-sha256`两种HMAC,比较`blake3`、`sha256`两种哈希算法的计算速度

两种hmac方法都分别实现了并行和串行两种计算方式，并进行benchmark

benchmark包含测试并行实现的速度、串行实现的速度和并行和串行的速度对比

## 如何运行速度测试

```sh
# 该命令会运行所有benchmark
cargo bench
```

## 如何运行`hmac-sha256`示例

```sh
cargo run --example <想要运行的例子>
```

## 定义的例子列表

- `hmac`
- `par_hmac_string_1_to_4_xor`
- `seq_hmac_string_1_to_4_xor`

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

其中`example`目录里的`hmac.rs`完全来自`hmac`官方文档[https://docs.rs/hmac/0.12.1/hmac] 。

`par_hmac**.rs`和`seq_hmac**.rs`参考了`hmac`官方文档[https://docs.rs/hmac/0.12.1/hmac] 的内容，分别是使用并行哈希和串行哈希实现的`hmac-sha256`。
