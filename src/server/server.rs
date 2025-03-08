use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use blockchainlib::{Blockchain, Block};
use blockchainlib::*;

// 新增交易请求结构
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    inputs: Vec<transaction::Output>,
    outputs: Vec<transaction::Output>,
}

#[derive(Clone)]
pub(crate) struct MyServer {
    pub(crate) address: String,
    pub(crate) blockchain: Arc<Mutex<Blockchain>>,
}

impl MyServer {
    pub fn new(address: &str, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        MyServer {
            address: address.to_string(),
            blockchain,
        }
    }

    pub(crate) async fn run(self) -> std::io::Result<()> {
        let blockchain = self.blockchain.clone();
        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec!["Content-Type", "Authorization", "X-Requested-With"]);

            App::new()
                .app_data(web::Data::new(blockchain.clone()))
                .wrap(cors)
                .route("/scan", web::get().to(Self::scan))
                .route("/data", web::get().to(Self::data))
                .route("/mine", web::post().to(Self::mine))
        })
            .bind(&self.address)?
            .run()
            .await
    }

    async fn scan(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
        let blockchain = data.lock().unwrap();
        let response = json!({
            "success": true,
            "message": "scan",
            "blocks": blockchain.blocks.len(),
        });
        HttpResponse::Ok().json(response)
    }

    async fn mine(
        data: web::Data<Arc<Mutex<Blockchain>>>,
        transactions: web::Json<Vec<TransactionRequest>>, // 接收交易数据
    ) -> impl Responder {
        let mut blockchain = match data.lock() {
            Ok(lock) => lock,
            Err(_) => return HttpResponse::InternalServerError().json(
                json!({ "success": false, "message": "无法获取区块链锁" })
            )
        };

        // 转换交易格式
        let transactions: Vec<Transaction> = transactions
            .into_inner()
            .into_iter()
            .map(|t| Transaction {
                inputs: t.inputs,
                outputs: t.outputs,
            })
            .collect();

        // 验证至少有一个交易
        if transactions.is_empty() {
            return HttpResponse::BadRequest().json(
                json!({ "success": false, "message": "至少需要包含一个交易" })
            );
        }

        let last_block = match blockchain.blocks.last() {
            Some(b) => b,
            None => return HttpResponse::InternalServerError().json(
                json!({ "success": false, "message": "区块链尚未初始化" })
            )
        };

        let mut new_block = Block::new(
            last_block.index + 1,
            now(),
            last_block.hash.clone(),
            transactions, // 使用传入的交易
            last_block.difficulty,
        );

        // 执行挖矿
        let mining_result = new_block.mine();
        if !mining_result.is_ok() {
            return HttpResponse::InternalServerError().json(
                json!({ "success": false, "message": "挖矿失败" })
            );
        }

        // 添加新区块
        match blockchain.update_with_block(new_block) {
            Ok(_) => HttpResponse::Ok().json(
                json!({ "success": true, "message": "新区块已挖出" })
            ),
            Err(e) => HttpResponse::InternalServerError().json(
                json!({ "success": false, "message": format!("添加区块失败: {:?}", e) })
            )
        }
    }

    // async fn mine(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    //     let mut blockchain = data.lock().unwrap();
    //     let last_block = blockchain.blocks.last().unwrap();
    //     let difficulty = last_block.difficulty;
    //
    //     let mut new_block = Block::new(
    //         last_block.index + 1,
    //         now() as u128,
    //         last_block.hash.clone(),
    //         vec![], // 这里可以添加实际交易
    //         difficulty,
    //     );
    //
    //     new_block.mine();
    //     blockchain.update_with_block(new_block).expect("挖矿失败");
    //     HttpResponse::Ok().json(json!({ "success": true, "message": "新区块已挖出" }))
    // }


    async fn data(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
        let blockchain = match data.lock() {
            Ok(lock) => lock,
            Err(_) => return HttpResponse::InternalServerError().json(
                json!({ "success": false, "message": "无法获取区块链锁" })
            )
        };

        // 收集所有交易数据
        let all_transactions: Vec<&Transaction> = blockchain.blocks
                                                            .iter()
                                                            .flat_map(|block| &block.transactions)
                                                            .collect();

        HttpResponse::Ok().json(json!({
            "success": true,
            "block_count": blockchain.blocks.len(),
            "total_transactions": all_transactions.len(),
            "transactions": all_transactions
        }))
    }

    // async fn data(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    //     let blockchain = data.lock().unwrap();
    //     HttpResponse::Ok().json(json!({
    //         "success": true,
    //         "blocks": blockchain.blocks
    //     }))
    // }
}
