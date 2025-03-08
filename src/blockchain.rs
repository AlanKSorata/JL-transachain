use super::*;
use std::collections::HashSet;

// 定义区块链验证错误
#[derive(Debug, PartialEq)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

// 定义区块链结构体
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

// 实现区块链结构体
impl Blockchain {
    // 创建新的区块链
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    // 更新区块链
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let block_num = self.blocks.len();

        // 检查区块是否有效
        if block.index != block_num as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if block_num != 0 {
            // 非 Genesis 区块
            let prev_block = &self.blocks[block_num - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis 区块
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        // 检查交易是否有效
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            // 检查 Coinbase 交易
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            // 检查双花问题
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            // 遍历区块中的交易
            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                // 检查输入是否有效且未被重复花费
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                // 计算输入和输出金额
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                // 输出金额不可超过输入金额
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                // 累加手续费
                let fee = input_value - output_value;
                total_fee += fee;

                // 记录已花费和新生成的 UTXO
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            // Coinbase 交易必须覆盖手续费
            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            // 更新 UTXO 集合
            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
