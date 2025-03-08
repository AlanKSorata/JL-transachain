use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use blockchainlib::{Block, Blockchain, Transaction, now, transaction};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

// 共享状态
struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
    difficulty: u128,
}

// 可序列化的交易结构
#[derive(Serialize, Deserialize)]
struct ApiTransaction {
    inputs: Vec<transaction::Output>,
    outputs: Vec<transaction::Output>,
}

// API响应格式
#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}
#[derive(Clone)]
pub(crate) struct MyServer {
    address: String,
}

impl MyServer {
    /// 初始化服务器
    pub fn new(address: &str) -> Self {
        MyServer {
            address: String::from(address),
        }
    }

    /// 启动服务器
    pub(crate) async fn run(self) -> std::io::Result<()> {
        // 初始化区块链
        let difficulty = 0x000fffffffffffffffffffffffffffff;
        let blockchain = Arc::new(Mutex::new(Self::create_genesis_blockchain(difficulty)));

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec!["Content-Type", "Authorization", "X-Requested-With"]),
                )
                .app_data(web::Data::new(AppState {
                    blockchain: blockchain.clone(),
                    difficulty,
                }))
                .route("/blocks", web::get().to(Self::get_blocks))
                .route("/mine", web::post().to(Self::mine_block))
                .route("/transaction", web::post().to(Self::add_transaction))
        })
        .bind(&self.address)?
        .run()
        .await
    }

    // 创建创世区块链
    fn create_genesis_blockchain(difficulty: u128) -> Blockchain {
        let mut genesis_block = Block::new(
            0,
            now(),
            vec![0; 32],
            vec![Transaction {
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
        genesis_block.mine();

        let mut blockchain = Blockchain::new();
        blockchain.update_with_block(genesis_block).unwrap();
        blockchain
    }

    // 获取区块链数据
    async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
        let blockchain = data.blockchain.lock().unwrap();

        let blocks: Vec<_> = blockchain
            .blocks
            .iter()
            .map(|block| {
                json!({
                    "index": block.index,
                    "timestamp": block.timestamp,
                    "hash": hex::encode(&block.hash),
                    "prev_hash": hex::encode(&block.prev_block_hash),
                    "transactions": block.transactions.len(),
                    "nonce": block.nonce
                })
            })
            .collect();

        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Blocks retrieved".to_string(),
            data: Some(json!({ "blocks": blocks })),
        })
    }

    // 挖矿端点
    async fn mine_block(data: web::Data<AppState>) -> impl Responder {
        let mut blockchain = data.blockchain.lock().unwrap();
        let difficulty = data.difficulty;

        let last_block = blockchain.blocks.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            now(),
            last_block.hash.clone(),
            vec![], // 需要收集未处理交易
            difficulty,
        );

        match blockchain.update_with_block(new_block) {
            Ok(_) => HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: "New block mined".to_string(),
                data: None,
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Mining failed: {:?}", e),
                data: None,
            }),
        }
    }

    // 添加交易端点
    async fn add_transaction(
        data: web::Data<AppState>,
        transaction: web::Json<ApiTransaction>,
    ) -> impl Responder {
        let mut blockchain = data.blockchain.lock().unwrap();

        // 转换API交易到库的交易类型
        let new_transaction = Transaction {
            inputs: transaction.inputs.clone(),
            outputs: transaction.outputs.clone(),
        };

        // 这里应该添加交易验证逻辑
        // 暂时直接添加到最新区块
        if let Some(block) = blockchain.blocks.last_mut() {
            block.transactions.push(new_transaction);
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: "Transaction added".to_string(),
                data: None,
            })
        } else {
            HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: "No blocks in chain".to_string(),
                data: None,
            })
        }
    }
}
