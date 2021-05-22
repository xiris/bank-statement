use actix_web::Responder;

pub async fn get_transactions() -> impl Responder {
    format!("Hello transactions")
}

pub async fn get_transaction_by_id() -> impl Responder {
    format!("Hello transaction by id")
}

pub async fn add_transaction() -> impl Responder {
    format!("Hello add transaction")
}

pub async fn delete_transaction() -> impl Responder {
    format!("Hello delete transaction")
}  