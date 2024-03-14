use actix_web::{web, HttpResponse, Responder};
use crate::models::user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::post().to(create_user));
}

async fn create_user(item: web::Json<user::User>) -> impl Responder {
    let user = item.into_inner(); // 获取请求体中的数据
    HttpResponse::Ok().json(user) // 响应：将接收到的数据返回给客户端
}