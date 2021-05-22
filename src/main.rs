use actix_web::{App, HttpServer, web};
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web-debug");

    HttpServer::new(move || {
        App::new()
            .route("/transactions", web::get().to(handlers::get_transactions))
            .route("/transactions/{id}", web::get().to(handlers::get_transaction_by_id))
            .route("/transactions", web::post().to(handlers::add_transaction))
            .route("/transactions", web::delete().to(handlers::delete_transaction))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}