use super::*;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashable() {
        // 定义测试结构体
        struct Test {
            a: u32,
            b: u32,
            c: u32,
        }

        // 实现 Hashable 特性
        impl Hashable for Test {
            fn bytes(&self) -> Vec<u8> {
                let mut bytes = vec![];
                bytes.extend(&u32_bytes(&self.a));
                bytes.extend(&u32_bytes(&self.b));
                bytes.extend(&u32_bytes(&self.c));
                bytes
            }
        }

        let test = Test { a: 1, b: 2, c: 3 };
        let hash = test.hash();
        assert_eq!(hash.len(), 32);
    }
}
