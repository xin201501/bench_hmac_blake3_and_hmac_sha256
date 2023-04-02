use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use crypto_bigint::U256;
use test_pal_hash::mock_generate_int;
fn seq_blake3_benchmark(c: &mut Criterion) {
    // let mut results = Vec::with_capacity(4);
    let data = mock_generate_int::<20>(4096);

    c.bench_with_input(
        BenchmarkId::new("Sequence blake3 hash bench", 0),
        &data,
        |b, data| {
            b.iter(move || {
                let map_val = data.iter().flatten().map(|i| {
                    let mut hasher = blake3::Hasher::new();
                    let mut mac_reader = hasher.update(&i.to_be_bytes()).finalize_xof();
                    let mut mac = [0; 32];
                    mac_reader.fill(&mut mac);
                    mac
                });
                let xor_value = map_val
                    // .inspect(|num|println!("{:?}",hex::encode(num)))
                    .fold(U256::default(), |a, b| {
                        let b = U256::from_be_slice(b.as_slice());
                        a.bitxor(&b)
                    });
                black_box(xor_value)
            })
        },
    );
}
criterion_group!(benches, seq_blake3_benchmark);
criterion_main!(benches);
