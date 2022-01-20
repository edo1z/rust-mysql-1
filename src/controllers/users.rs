use crate::db;
use actix_web::{HttpRequest, HttpResponse, Responder, web, Result};

pub async fn index(_req: HttpRequest) -> Result<impl Responder> {
    let users = web::block(move || db::user::find_all()).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("/users/add")
}
