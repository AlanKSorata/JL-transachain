use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// 定义交易输出结构体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub receiver: Address,
    pub value: u64,
}

// 实现 Hashable 特性
impl Hashable for Output {
    // 获取字节
    fn bytes(&self) -> Vec<u8> {
        // 创建字节数组
        let mut bytes = vec![];

        // 添加地址字节
        bytes.extend(self.receiver.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

// 定义交易结构体
#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

// 实现交易结构体
impl Transaction {
    // 获取输入金额
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    // 获取输出金额
    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    // 获取输入哈希
    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    // 获取输出哈希
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    // 是否为 Coinbase 交易
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

// 为交易结构体实现 Hashable 特性
impl Hashable for Transaction {
    // 获取字节
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        // 添加输入字节
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        // 添加输出字节
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        // 创建两个输出
        let output1 = Output {
            receiver: Address::new(),
            value: 100,
        };
        let output2 = Output {
            receiver: Address::new(),
            value: 200,
        };
        // 创建交易
        let transaction = Transaction {
            inputs: vec![output1.clone(), output2.clone()],
            outputs: vec![output1.clone(), output2.clone()],
        };

        // 测试交易的输入和输出金额
        assert_eq!(transaction.input_value(), 300);
        assert_eq!(transaction.output_value(), 300);
        assert_eq!(transaction.input_hashes().len(), 2);
        assert_eq!(transaction.output_hashes().len(), 2);
        assert_eq!(transaction.is_coinbase(), false);
    }
}
