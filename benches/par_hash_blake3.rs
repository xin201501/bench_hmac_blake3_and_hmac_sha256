use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypto_bigint::U256;
use rayon::prelude::*;
use test_pal_hash::mock_generate_int;
fn pal_blake3_benchmark(c: &mut Criterion) {
    let mock_data = mock_generate_int::<20>(100000);

    c.bench_function("Parralel blake3 hash bench", |b| {
        b.iter(|| {
            let result = mock_data
                .par_iter()
                .flat_map(|msgs| {
                    msgs.par_iter().map(|msg| {
                        let mut hasher = blake3::Hasher::new();
                        let mut mac_reader = hasher.update(&msg.to_be_bytes()).finalize_xof();
                        let mut mac = [0; 32];
                        mac_reader.fill(&mut mac);
                        // .with_context(|| "HMAC can take key of any size").unwrap();
                        mac
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
criterion_group!(benches, pal_blake3_benchmark);
criterion_main!(benches);
