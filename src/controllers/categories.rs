use crate::db;
use actix_web::{HttpRequest, HttpResponse};

pub async fn index(_req: HttpRequest) -> HttpResponse {
    let categories = db::category::find_all();
    let body = serde_json::to_string(&categories).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub async fn add(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("/categories/add")
}
