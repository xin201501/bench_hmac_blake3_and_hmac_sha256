use anyhow::Result;
use crypto_bigint::U256;

use rayon::prelude::*;
use ring::rand;
use std::ops::BitXor;
use test_pal_hash::mock_generate_int;
fn main() -> Result<()> {
    let _generator = rand::SystemRandom::new();
    let _key = b"my secret and secure key";
    // generator.fill(&mut key).unwrap();

    let mock_data = mock_generate_int::<20>(100000);
    let result = mock_data
        .par_iter()
        .flat_map(|msgs| {
            // let style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap();
            msgs.par_iter()
                // .progress_count(mock_data.len() as u64)
                // .progress_with_style(style)
                .map(|msg| {
                    let mut hasher = blake3::Hasher::new();
                    let mut mac_reader = hasher.update(&msg.to_be_bytes()).finalize_xof();
                    let mut mac = [0; 32];
                    mac_reader.fill(&mut mac);
                    // .with_context(|| "HMAC can take key of any size").unwrap();
                    mac
                })
            //     .fold(
            //         || U256::default(),
            //         |a, b| {

            //             let b = U256::from_be_slice(&b);
            //             a.bitxor(&b)
            //         }
            // )
            // .reduce(||U256::default(),|a,b|a.bitxor(&b))
        })
        .fold(
            || U256::default(),
            |a, b| {
                let b = U256::from_be_slice(&b);
                a.bitxor(&b)
            },
        )
        .reduce(|| U256::default(), |a, b| a.bitxor(&b));
    println!("{result}");
    Ok(())
}

// fn main(){
//     let data = mock_generate_int::<20>(100000);
//     let map_val = data.iter().flatten().map(|i| {
//         let mut mac = HmacSha256::new_from_slice(b"my secret and secure key").unwrap();
//         mac.update(&i.to_be_bytes());
//         mac.finalize_fixed()
//     });
//     let xor_value = map_val
//         // .inspect(|num|println!("{:?}",hex::encode(num)))
//         .fold(U256::default(), |a, b| {
//             let b = U256::from_be_slice(b.as_slice());
//             a.bitxor(&b)
//         });
//         println!("{xor_value}")
// }
