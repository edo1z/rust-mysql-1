#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};

mod controllers;
mod db;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
