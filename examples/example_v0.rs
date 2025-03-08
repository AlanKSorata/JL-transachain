use blockchainlib::{Block, Blockchain, Transaction, now, transaction};

#[allow(unused_assignments)]
fn main() {
    // 定义困难度
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    // 创建创世区块
    let mut genesis_block = Block::new(
        0,           // 索引
        now(),       // 时间戳
        vec![0; 32], // 前一个区块的哈希
        vec![Transaction {
            // 交易
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    receiver: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    receiver: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    // 挖掘创世区块
    genesis_block.mine();

    println!(
        "挖掘创世区块 {:?}\nInfo: {:#?}",
        &genesis_block, &genesis_block.transactions
    );

    // 记录上一个区块的哈希
    let mut last_hash = genesis_block.hash.clone();

    // 创建区块链
    let mut blockchain = Blockchain::new();

    // 更新区块链
    blockchain
        .update_with_block(genesis_block)
        .expect("添加创世区块失败");

    // 创建新区块
    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    receiver: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        receiver: "Alice".to_owned(),
                        value: 37,
                    },
                    transaction::Output {
                        receiver: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );

    // 挖掘新区块
    block.mine();

    println!("挖掘区块 {:?}\nInfo: {:#?}\n", &block, &block.transactions);

    last_hash = block.hash.clone();

    // 更新区块链
    blockchain.update_with_block(block).expect("添加区块失败");

    // println!(">> blocks: {:?}", &blockchain.blocks);

    // =====================================================================

    // 创建新区块
    block = Block::new(
        2,
        now(),
        last_hash,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    receiver: "Alice".to_owned(),
                    value: 3,
                },
                transaction::Output {
                    receiver: "Bob".to_owned(),
                    value: 12,
                },
            ],
        }],
        difficulty,
    );

    // 挖掘新区块
    block.mine();

    println!("挖掘区块 {:?}\nInfo: {:#?}\n", &block, &block.transactions);

    last_hash = block.hash.clone();

    // 更新区块链
    blockchain.update_with_block(block).expect("添加区块失败");
}
