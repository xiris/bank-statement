use actix_web::Responder;

use super::models::{TransactionNew, Transaction};
use super::schema::transactions::dsl::*;
use super::Pool;
use crate::{diesel::QueryDsl, models::TransactionJson};
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

// GET /transactions
pub async fn get_transactions(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_transactions(db))
        .await
        .map(|transaction| HttpResponse::Ok().json(transaction))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// GET /transactions/{id}
pub async fn get_transaction_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_transaction_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// POST /transactions
pub async fn add_transaction(
    db: web::Data<Pool>,
    item: web::Json<TransactionJson>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_transaction(db, item))
        .await
        .map(|transaction| HttpResponse::Created().json(transaction))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// DELETE /transactions/{id}
pub async fn delete_transaction(
    db: web::Data<Pool>,
    transaction_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_transaction(db, transaction_id.into_inner()))
            .await
            .map(|transaction| HttpResponse::Ok().json(transaction))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Functions

fn get_all_transactions(pool: web::Data<Pool>) -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let data = transactions.load::<Transaction>(&conn)?;
    Ok(data)
}

fn db_get_transaction_by_id(pool: web::Data<Pool>, transaction_id: i32) -> Result<Transaction, diesel::result::Error> {
    let conn = pool.get().unwrap();
    transactions.find(transaction_id).get_result::<Transaction>(&conn)
}

fn add_single_transaction(
    db: web::Data<Pool>,
    item: web::Json<TransactionJson>,
) -> Result<Transaction, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_transaction = TransactionNew {
        booking_date: &item.booking_date,
        value_date: &item.value_date,
        booking_text: &item.booking_text,
        beneficiary: &item.beneficiary,
        purpose: &item.purpose,
        account_number: &item.account_number,
        sort_code: &item.sort_code,
        amount: &item.amount,
        creditor_id: &item.creditor_id,
        mandate_reference: &item.mandate_reference,
        customer_reference: &item.customer_reference,
    };
    
    insert_into(transactions)
        .values(&new_transaction)
        .execute(&conn)
        .expect("Error adding transaction");
    
    let res = transactions.order(id).first(&conn).unwrap();
    Ok(res)
}

fn delete_single_transaction(db: web::Data<Pool>, transaction_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(transactions.find(transaction_id)).execute(&conn)?;
    Ok(count)
}