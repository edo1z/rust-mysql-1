use crate::db;
use actix_web::{HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
    let users = db::user::find_all();
    let body = serde_json::to_string(&users).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub fn add(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("/users/add")
}
