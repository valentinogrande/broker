use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::{Aead, Payload}};
use rand::rngs::OsRng;
use rand::TryRngCore;
use bip39::{Mnemonic, Language};
use sqlx::{MySqlPool,mysql::MySqlQueryResult};
use std::fs;
use actix_web::web;
use anyhow::{Error, anyhow};

use crate::structs::User;

#[allow(dead_code)]
impl User {
    pub async fn generate_wallet(&self, pool: web::Data<MySqlPool>) -> Result<MySqlQueryResult, Error>{
        
        let mut bytes = [0u8; 32];
        let mut rng = OsRng;
        let _ = rng.try_fill_bytes(&mut bytes);
        
        let seed = Mnemonic::from_entropy_in(Language::English, &bytes).unwrap();
     
        let mut dek = [0u8; 32];
        let _ = OsRng.try_fill_bytes(&mut dek);
        
        let mut nonce_bytes_seed = [0u8; 12];
        let _ = OsRng.try_fill_bytes(&mut nonce_bytes_seed);
        let nonce_seed = Nonce::from_slice(&nonce_bytes_seed);
        
        let cipher = Aes256Gcm::new_from_slice(&dek)
        .expect("DEK must be 32 bytes");

        let payload = Payload{
            msg: &seed.to_entropy_array().0,
            aad: &[],
        };
        let encrypted_seed = cipher
            .encrypt(nonce_seed, payload)
            .map_err(|e| anyhow!("seed encryption failed: {:?}", e))?;

        let kek = fs::read_to_string("/etc/broker/master.key")?.trim().to_string();
        let kek_version = fs::read_to_string("/etc/broker/master.key.v")?.trim().to_string();
        
        let kek_bytes = hex::decode(kek).unwrap();

        let mut nonce_bytes_dek = [0u8; 12];
        let _ = OsRng.try_fill_bytes(&mut nonce_bytes_dek);
        let nonce_dek = Nonce::from_slice(&nonce_bytes_dek);

        let dek_cipher = Aes256Gcm::new_from_slice(&kek_bytes)
            .expect("KEK must be 32 bytes");


        let payload = Payload {
            msg: &dek,
            aad: &[],
        };

        let encrypted_dek = dek_cipher
            .encrypt(nonce_dek, payload)
            .map_err(|e| anyhow!("DEK encryption failed: {:?}", e))?;

        let query = sqlx::query("insert into wallets (user_id, encrypted_seed, seed_nonce, encrypted_dek, dek_nonce, kek_version)")
            .bind(&self.id)
            .bind(encrypted_seed)
            .bind(&nonce_bytes_seed[..])
            .bind(encrypted_dek)
            .bind(&nonce_bytes_dek[..])
            .bind(kek_version)
            .execute(pool.get_ref())
            .await;
        
        Ok(query?)
    }
}
