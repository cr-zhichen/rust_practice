use actix_web::web;
use crate::routes::users_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users").configure(users_service::configure));
}
