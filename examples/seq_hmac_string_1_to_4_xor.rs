//！参考[hmac]官方文档 https://docs.rs/hmac/0.12.1/hmac
use crypto_bigint::U256;
use hmac::{digest::FixedOutput, Hmac, Mac};
use sha2::Sha256;
// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;
fn main() {
    let mut results = Vec::with_capacity(4);
    for i in [1.to_string(), 2.to_string(), 3.to_string(), 4.to_string()] {
        println!("the byte representation of i is : {:?}", i.as_bytes());
        let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
            .expect("HMAC can take key of any size");
        mac.update(i.as_bytes());
        let result = mac.finalize_fixed();
        println!("the HMAC value of i is: {}", hex::encode(result));
        results.push(result)
    }
    let xor_value = results
        .iter()
        // .inspect(|num|println!("{:?}",hex::encode(num)))
        .fold(U256::default(), |a, b| {
            let b = U256::from_be_slice(b.as_slice());
            a.bitxor(&b)
        });
    println!("the final hash result is: {xor_value}")
}
