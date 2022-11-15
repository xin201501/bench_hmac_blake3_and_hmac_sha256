use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crypto_bigint::U256;
use hmac::{digest::FixedOutput, Hmac, Mac};
use rayon::prelude::*;
use sha2::Sha256;
use test_pal_hash::mock_generate_int;
type HmacSha256 = Hmac<Sha256>;
fn pal_hmac_benchmark(c: &mut Criterion) {
    let mock_data = mock_generate_int::<20>(4096);
   
    c.bench_function("parralel HMAC bench", |b| {
        b.iter(|| {
            let key = b"my secret and secure key";
            let result = mock_data
                .par_iter()
                .flat_map(|msgs| {
                    msgs.par_iter().map(|msg| {
                        let mut mac = HmacSha256::new_from_slice(key).unwrap();
                        mac.update(&msg.to_be_bytes());
                        mac.finalize_fixed()
                    })
                })
                .fold(
                    || U256::default(),
                    |a, b| {
                        let b = U256::from_be_slice(&b);
                        a.bitxor(&b)
                    },
                )
                .reduce(|| U256::default(), |a, b| a.bitxor(&b));
            black_box(result);
        })
    });
}
criterion_group!(benches, pal_hmac_benchmark);
criterion_main!(benches);
