use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::{Aead, Payload}};
use rand::rngs::OsRng;
use rand::TryRngCore;
use sqlx::{MySqlPool,mysql::MySqlQueryResult};
use actix_web::web;
use anyhow::{Error, anyhow};
use zeroize::Zeroizing;

use crate::{impl_user::crypto::get_kek::{get_kek_version, get_kek}, structs::User};

#[allow(dead_code)]
impl User {
    pub async fn generate_wallet(&self, pool: web::Data<MySqlPool>) -> Result<MySqlQueryResult, Error>{
        

        let mut entropy = Zeroizing::new([0u8; 32]);
        OsRng.try_fill_bytes(&mut *entropy)
            .map_err(|_| anyhow!("CSPRNG failed"))?;

        let mut dek = Zeroizing::new([0u8; 32]);
        OsRng.try_fill_bytes(&mut (*dek)).map_err(|_| anyhow!("CSPRNG failed"))?;
        
        let mut nonce_bytes_entropy = [0u8; 12];
        let _ = OsRng.try_fill_bytes(&mut nonce_bytes_entropy);
        let nonce_entropy = Nonce::from_slice(&nonce_bytes_entropy);

        let aad = self.id.to_be_bytes();
        
        let encrypted_entropy = {
            let cipher = Aes256Gcm::new_from_slice(&(*dek))
            .expect("DEK must be 32 bytes");


            let payload = Payload{
                msg: &(*entropy),
                aad: &aad,
            };

            let encrypted_entropy = cipher
                .encrypt(nonce_entropy, payload)
                .map_err(|e| anyhow!("entropy encryption failed: {:?}", e))?;
            encrypted_entropy   
        };
        
        let mut nonce_bytes_dek = [0u8; 12];
        let _ = OsRng.try_fill_bytes(&mut nonce_bytes_dek);
        let nonce_dek = Nonce::from_slice(&nonce_bytes_dek);
        
        let encrypted_dek = {

            let kek: Zeroizing<Vec<u8>> = get_kek()?;
            let dek_cipher = Aes256Gcm::new_from_slice(&kek)
                .map_err(|_| anyhow!("Invalid DEK length"))?;


            let payload = Payload {
                msg: &(*dek),
                aad: &aad,
            };

            let encrypted_dek = dek_cipher
                .encrypt(nonce_dek, payload)
                .map_err(|e| anyhow!("DEK encryption failed: {:?}", e))?;
            encrypted_dek
        };
        
        let kek_version = get_kek_version()?;

        let query = sqlx::query("insert into wallets (user_id, encrypted_entropy, entropy_nonce, encrypted_dek, dek_nonce, kek_version)")
            .bind(&self.id)
            .bind(encrypted_entropy)
            .bind(&nonce_bytes_entropy[..])
            .bind(encrypted_dek)
            .bind(&nonce_bytes_dek[..])
            .bind(kek_version)
            .execute(pool.get_ref())
            .await;
        
        Ok(query?)
    }
}
