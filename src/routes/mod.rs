use crate::controllers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(controllers::index))
        .service(web::resource("/users").to(controllers::users::index))
        .service(web::resource("/users/add").to(controllers::users::add))
        .service(web::resource("/categories").to(controllers::categories::index))
        .service(web::resource("/categories/add").to(controllers::categories::add))
        .service(web::resource("/posts").to(controllers::posts::index))
        .service(web::resource("/posts/add").to(controllers::posts::add));
}
