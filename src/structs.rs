use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use chrono::NaiveDate;
use crate::enums::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Creds {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct EncryptedWallet {
    encrypted_seed: String,
    seed_nonce: String,
    encrypted_dek: String,
    dek_nonce: String,
    kek_version: u8,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct UserData {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub photo: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct NewIndividual {
    pub address: String,
    pub name: String,
    pub birth_date: NaiveDate,
    pub trading_experience: TradingExperience,
    pub pep: bool,
    pub annual_income: AnnualIncome,
    pub transaction_amount: TransactionAmount,
    pub is_owner_beneficiary: bool,
    pub source_of_funds: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewCompany {
    pub company_name: String,
    pub registration_number: u64,
    pub country: String, //ISO 3166-1 
    pub status: Status,
    pub company_type: String,
    pub nature_of_business: String,
    pub incorporation_date: NaiveDate,
    pub registered_address: String,
}
