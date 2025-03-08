mod server;

use server::server::MyServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = MyServer::new("0.0.0.0:10000");
    println!("server running on http://0.0.0.0:10000");
    server.run().await
}
