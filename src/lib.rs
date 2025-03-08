type Hash = Vec<u8>;
type Address = String;

use std::time::{SystemTime, UNIX_EPOCH};

// 获取当前时间戳
pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    // 返回时间戳
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

// 获取 u32 类型的字节数组
pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

// 获取 u64 类型的字节数组
pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

// 获取 u128 类型的字节数组
pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

// 获取困难度字节数组
pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    assert!(
        v.len() >= 16,
        "The input vector must have at least 16 bytes"
    );

    let mut result = 0u128;
    for (i, byte) in v.iter().rev().take(16).enumerate() {
        result |= (*byte as u128) << (i * 8);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_bytes() {
        assert_eq!(u32_bytes(&0x12345678), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_u64_bytes() {
        assert_eq!(
            u64_bytes(&0x0123456789abcdef),
            [0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01]
        );
    }

    #[test]
    fn test_u128_bytes() {
        assert_eq!(
            u128_bytes(&0x0123456789abcdef0123456789abcdef),
            [
                0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45,
                0x23, 0x01
            ]
        );
    }

    #[test]
    fn test_difficulty_bytes_as_u128() {
        // 注意是大端序
        assert_eq!(
            difficulty_bytes_as_u128(&vec![
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef
            ]),
            0x0123456789abcdef0123456789abcdef
        );
    }
}

mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;
mod blockchain;
pub use crate::blockchain::Blockchain;
pub mod transaction;
pub use crate::transaction::Transaction;
