use sqlx::Type;
use serde::{Serialize, Deserialize};

#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "trading_experience")]
#[sqlx(rename_all = "lowercase")]        
pub enum TradingExperience {
    None,
    Basic,
    Intermediate,
    Advanced,
    Professional,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "annual_income")]
#[sqlx(rename_all = "lowercase")] 
pub enum AnnualIncome {
    #[sqlx(rename = "<10k")]
    Lt10k,
    #[sqlx(rename = "10k-25k")]
    K10To25,
    #[sqlx(rename = "25k-50k")]
    K25To50,
    #[sqlx(rename = "50k-100k")]
    K50To100,
    #[sqlx(rename = "100k-250k")]
    K100To250,
    #[sqlx(rename = ">250k")]
    Gt250k,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_amount")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionAmount {
    #[sqlx(rename = "<1k")]
    Lt1k,
    #[sqlx(rename = "1k-5k")]
    K1To5,
    #[sqlx(rename = "5k-20k")]
    K5To20,
    #[sqlx(rename = "20k-50k")]
    K20To50,
    #[sqlx(rename = ">50k")]
    Gt50,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_amount")]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Active,
    Inactive
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub enum Subject {
    Company,
    Individual,
}

