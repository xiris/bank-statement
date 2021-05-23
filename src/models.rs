use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Transaction {
    pub id: Option<i32>,
    pub booking_date: String,
    pub value_date: String,
    pub booking_text: String,
    pub beneficiary: String,
    pub purpose: String,
    pub account_number: String,
    pub sort_code: String,
    pub amount: String,
    pub creditor_id: String,
    pub mandate_reference: String,
    pub customer_reference: String,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct TransactionNew<'a> {
    pub booking_date: &'a str,
    pub value_date: &'a str,
    pub booking_text: &'a str,
    pub beneficiary: &'a str,
    pub purpose: &'a str,
    pub account_number: &'a str,
    pub sort_code: &'a str,
    pub amount: &'a str,
    pub creditor_id: &'a str,
    pub mandate_reference: &'a str,
    pub customer_reference: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionJson {
    pub booking_date: String,
    pub value_date: String,
    pub booking_text: String,
    pub beneficiary: String,
    pub purpose: String,
    pub account_number: String,
    pub sort_code: String,
    pub amount: String,
    pub creditor_id: String,
    pub mandate_reference: String,
    pub customer_reference: String,
}
