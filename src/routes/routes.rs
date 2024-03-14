use actix_web::web;
use crate::handlers::{users};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users").configure(users::configure));
}
