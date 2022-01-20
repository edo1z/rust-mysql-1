use actix_web::{HttpRequest, HttpResponse};

pub mod categories;
pub mod posts;
pub mod users;

pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}
