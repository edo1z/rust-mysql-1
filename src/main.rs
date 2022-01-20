use actix_web::{get, App, Responder, HttpResponse, HttpServer};

mod controllers;
mod models;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello2")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().service(index);
        app
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}