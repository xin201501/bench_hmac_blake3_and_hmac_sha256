use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crypto_bigint::U256;
use hmac::{digest::FixedOutput, Hmac, Mac};
use rayon::prelude::*;
use sha2::Sha256;
use test_pal_hash::{mock_generate_string, mock_generate_int};
type HmacSha256 = Hmac<Sha256>;
fn seq_hmac_benchmark(c: &mut Criterion) {
    // let mut results = Vec::with_capacity(4);
    let data = mock_generate_int::<8>(2048);

    c.bench_with_input(BenchmarkId::new("seq HMAC bench", 0), &data, |b, data| {
        b.iter(move || {
            let map_val = data.iter().flatten().map(|i| {
                let mut mac = HmacSha256::new_from_slice(b"my secret and secure key").unwrap();
                mac.update(&i.to_be_bytes());
                mac.finalize_fixed()
            });
            let xor_value = map_val
                // .inspect(|num|println!("{:?}",hex::encode(num)))
                .fold(U256::default(), |a, b| {
                    let b = U256::from_be_slice(b.as_slice());
                    a.bitxor(&b)
                });
            black_box(xor_value)
        })
    });
}
criterion_group!(benches, seq_hmac_benchmark);
criterion_main!(benches);
