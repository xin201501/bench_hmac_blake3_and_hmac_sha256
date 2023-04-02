use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypto_bigint::U256;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use test_pal_hash::mock_generate_int;
fn pal_sha256_benchmark(c: &mut Criterion) {
    let mock_data = mock_generate_int::<20>(100000);
    c.bench_function("Parralel sha256 hash bench", |b| {
        b.iter(|| {
            let result = mock_data
                .par_iter()
                .flat_map(|msgs| {
                    msgs.par_iter().map(|msg| {
                        let mut mac = Sha256::new();
                        mac.update(&msg.to_be_bytes());
                        mac.finalize()
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
criterion_group!(benches, pal_sha256_benchmark);
criterion_main!(benches);
