use crate::structs::User;
use actix_web::web;
use sqlx::MySqlPool;
use anyhow::{Error, anyhow};

impl User{
    pub async fn start_kyc(&self, pool: web::Data<MySqlPool>) -> Result<(), Error>{
        
        //0. check for existance on db whose status is 'not_started'
        let query: bool = sqlx::query_scalar("select exists (select 1 from kyc where user_id = ? and status = 'not_started')")
            .bind(self.id)
            .fetch_one(pool.get_ref())
            .await?; 

        if !query {
            return Err(anyhow!("kyc status is already iniziatilated".to_string()))
        }
        
        Ok(())
    }
}


        //1. ask for documents
        //2. req to real/sandbox API
        //3. insert into kyc the pending status

