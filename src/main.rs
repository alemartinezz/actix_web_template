use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod handlers;
use crate::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/register", web::post().to(register_user))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
