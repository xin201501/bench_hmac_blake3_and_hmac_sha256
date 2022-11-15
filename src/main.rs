use anyhow::{Context,Result};
use hmac::{digest::FixedOutput, Hmac, Mac};
use rayon::prelude::*;
use ring::rand::{self};
use sha2::Sha256;
use std::ops::BitXor;
use crypto_bigint::U256;
use test_pal_hash::mock_generate_int;
// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;
fn main() -> Result<()> {
    let _generator = rand::SystemRandom::new();
    let  key = b"my secret and secure key";
    // generator.fill(&mut key).unwrap();

    
    let mock_data = mock_generate_int::<20>(100000);
   

    let result = mock_data
        .par_iter()
        .flat_map(|msgs| {
            msgs.par_iter().map(|msg| {
                let mut mac = HmacSha256::new_from_slice(key)
                .with_context(|| "HMAC can take key of any size").unwrap();
                mac.update(&msg.to_be_bytes());
                mac.finalize_fixed()
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
                }
        )
        .reduce(||U256::default(),|a,b|a.bitxor(&b));
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