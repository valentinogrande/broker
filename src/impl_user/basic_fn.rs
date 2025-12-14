use crate::structs::{NewCompany, NewIndividual, User};
use sqlx::MySqlPool;
use anyhow::{anyhow, Error};

impl User {
    pub fn new(id: u64) -> Self {
        User{
        id
        }
    }
    pub async fn is_admin(&self, pool: &MySqlPool) -> Result<bool, sqlx::Error> {
        let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM users WHERE id = ?")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(is_admin)
    }
    

    // pub async fn get_subject(&self, pool: &MySqlPool) -> Result<Subject, sqlx::Error> {}
    

    pub async fn create_individual(&self, pool: &MySqlPool, individual: NewIndividual) -> Result<(), Error> {
        
        //1. verification
        let query: bool = sqlx::query_scalar("select exists (select 1 from companies where user_id = ?)")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        if query {
            return Err(anyhow!("a company with this user id already exist"))
        }
        
        //2. insertion
        sqlx::query(
            r#"
            insert into individuals 
            (user_id, address, name, birth_date, trading_experience, pep, annual_income, transaction_amount, is_owner_beneficary, source_of_funds)
            values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
            ).bind(self.id)
            .bind(&individual.address)
            .bind(&individual.name)
            .bind(&individual.birth_date)
            .bind(&individual.trading_experience)
            .bind(&individual.pep)
            .bind(&individual.annual_income)
            .bind(&individual.transaction_amount)
            .bind(&individual.is_owner_beneficiary)
            .bind(&individual.source_of_funds)
            .execute(pool)
            .await?;
        
        Ok(())

    }

    pub async fn create_company(&self, pool: &MySqlPool, company: NewCompany) -> Result<(), Error> {
        
        //1. verification
        let query: bool = sqlx::query_scalar("select exists (select 1 from individuals where user_id = ?)")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        if query {
            return Err(anyhow!("an individual with this user id already exist"))
        }

        //2. insertion
        sqlx::query(
            r#"
            insert into companies
            (user_id, company_name, registration_number, country, status, company_type, nature_of_business, incorporation_date, registered_address) 
            values (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
            ).bind(self.id)
            .bind(&company.company_name)
            .bind(&company.registration_number)
            .bind(&company.country)
            .bind(&company.status)
            .bind(&company.company_type)
            .bind(&company.nature_of_business)
            .bind(&company.incorporation_date)
            .bind(&company.registered_address)
            .execute(pool)
            .await?;

        Ok(())
    }

}
