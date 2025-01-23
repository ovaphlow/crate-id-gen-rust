use std::time::{SystemTime, UNIX_EPOCH};
use rand::RngCore;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};
use num_integer::Integer;

const KSUID_BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn base62_encode(data: &[u8]) -> String {
    let mut value = BigUint::from_bytes_be(data);
    let base = BigUint::from(62u32);
    let zero = BigUint::zero();
    let mut result = Vec::new();

    while value > zero {
        let (quotient, remainder) = value.div_rem(&base);
        result.push(KSUID_BASE62[remainder.to_usize().unwrap()]);
        value = quotient;
    }

    while result.len() < 27 {
        result.push(b'0');
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    random_bytes
}

pub fn generate_ksuid() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let mut timestamp_bytes = [0u8; 4];
    timestamp_bytes.copy_from_slice(&timestamp.to_be_bytes());

    let random_bytes = generate_random_bytes(16);

    let mut ksuid_bytes = Vec::with_capacity(20);
    ksuid_bytes.extend_from_slice(&timestamp_bytes);
    ksuid_bytes.extend_from_slice(&random_bytes);

    base62_encode(&ksuid_bytes)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_generate_ksuid() {
        let ksuid_str = generate_ksuid();

        // 输出生成的 KSUID 字符串
        println!("Generated KSUID: {}", ksuid_str);

        assert_eq!(ksuid_str.len(), 27);

        // Ensure the KSUID is base62 encoded
        for c in ksuid_str.chars() {
            assert!(KSUID_BASE62.contains(&(c as u8)));
        }
    }
}
