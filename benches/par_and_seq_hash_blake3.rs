use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use crypto_bigint::U256;
use rayon::prelude::*;
use test_pal_hash::mock_generate_string;

fn par_and_seq_hmac(c: &mut Criterion) {
    let mut group = c.benchmark_group("par and seq hash blake3");
    // let mock_data = mock_generate_string::<2>(2048);
    for size in 20..22 {
        let data_size = 1 << size;
        group.throughput(Throughput::Bytes(data_size));
        let input = mock_generate_string::<2>(data_size as usize);
        group.bench_with_input(
            BenchmarkId::new("Parallel", data_size),
            &input,
            |b, input| {
                // let mut messages_vector = Vec::with_capacity(input.len());
                // for data in input {
                //     // messages_vector.push(generate_object_collections::<2>());
                //     messages_vector.push(data);
                // }
                b.iter(|| {
                    // let key = b"my secret and secure key";
                    let result = input
                        .par_iter()
                        .flat_map(|msgs| {
                            msgs.par_iter().map(|msg| {
                                let mut hasher = blake3::Hasher::new();
                                let mut mac_reader = hasher.update(msg.as_bytes()).finalize_xof();
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
            },
        );
        // let mock_data: Vec<_> = mock_data.iter().flatten().collect();
        group.bench_with_input(
            BenchmarkId::new("Sequence", data_size),
            &input,
            |b, mock_data| {
                b.iter(|| {
                    let map_val = mock_data.iter().flatten().map(|i| {
                        let mut hasher = blake3::Hasher::new();
                        let mut mac_reader = hasher.update(i.as_bytes()).finalize_xof();
                        let mut mac = [0; 32];
                        mac_reader.fill(&mut mac);
                        // .with_context(|| "HMAC can take key of any size").unwrap();
                        mac
                        // results.push(result)
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
}
criterion_group!(benches, par_and_seq_hmac);
criterion_main!(benches);
