use actix_web::{HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("/categories/index")
}

pub fn add(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("/categories/add")
}
