use anyhow::Error;
use sqlx::MySqlPool;

use crate::structs::User;
use crate::enums::Subject;

impl User {
    pub async fn create_kyc(&self, pool: &MySqlPool, subject: Subject) -> Result<(), Error>{
        match subject {
            
            Subject::Company => {
                sqlx::query("insert into kyc (user_id, subject_type) values (?,?)")
                .bind(self.id)
                .bind(Subject::Company)
                .execute(pool)
                .await?;
            Ok(())
            }

            Subject::Individual => {
                sqlx::query("insert into kyc (user_id) values (?)")
                .bind(self.id)
                .execute(pool)
                .await?;
            Ok(())
            }

        }
    }
}
