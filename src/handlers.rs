use actix_web::Responder;

use super::models::{TransactionNew, Transaction};
use super::schema::transactions::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub async fn get_transactions(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_transactions(db))
        .await
        .map(|transaction| HttpResponse::Ok().json(transaction))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_transactions(pool: web::Data<Pool>) -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let data = transactions.load::<Transaction>(&conn)?;
    Ok(data)
}

pub async fn get_transaction_by_id() {}
pub async fn add_transaction() {}
pub async fn delete_transaction() {}