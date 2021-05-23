#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, App, HttpServer, web, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web-debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to connect to pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/transactions", web::get().to(handlers::get_transactions))
            .route("/transactions/{id}", web::get().to(handlers::get_transaction_by_id))
            .route("/transactions", web::post().to(handlers::add_transaction))
            .route("/transactions", web::delete().to(handlers::delete_transaction))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}