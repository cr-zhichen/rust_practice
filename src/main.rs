use std::env;
use actix_web::{App, HttpServer};

mod routes;
mod models;
mod config;
mod errors;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 加载.env文件中的环境变量
    dotenv::dotenv().ok();

    let listen_address = env::var("LISTENING_ADDRESS").expect("LISTENING_ADDRESS 必须设置");

    HttpServer::new(|| {
        App::new()
            .configure(routes::routes::configure)
    })
        .bind(listen_address)?
        .run()
        .await
}
