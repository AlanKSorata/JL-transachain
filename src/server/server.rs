use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde_json::json;

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
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec!["Content-Type", "Authorization", "X-Requested-With"]),
                )
                .route("/scan", web::get().to(Self::scan))
                .route("/create", web::get().to(Self::create))
                .route("/mine", web::get().to(Self::mine))
                .route("/data", web::get().to(Self::data))
        })
        .bind(&self.address)?
        .run()
        .await
    }

    /// scan
    async fn scan() -> impl Responder {
        let mut response = json!({
            "success": false,
            "message": ""
        });
        response["success"] = json!(true);
        response["message"] = json!("scan");
        HttpResponse::Ok().json(response)
    }

    /// create
    async fn create() -> impl Responder {
        let mut response = json!({
            "success": false,
            "message": ""
        });
        response["success"] = json!(true);
        response["message"] = json!("create");
        HttpResponse::Ok().json(response)
    }

    /// mine
    async fn mine() -> impl Responder {
        let mut response = json!({
            "success": false,
            "message": ""
        });
        response["success"] = json!(true);
        response["message"] = json!("mine");
        HttpResponse::Ok().json(response)
    }

    /// data
    async fn data() -> impl Responder {
        let mut response = json!({
            "success": false,
            "message": ""
        });
        response["success"] = json!(true);
        response["message"] = json!("data");
        HttpResponse::Ok().json(response)
    }
}
