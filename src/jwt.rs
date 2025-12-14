use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Serialize, Deserialize};
use std::fs;

use crate::structs::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user: User,
}

#[allow(dead_code)]
impl Claims {
    pub fn new(user: User) -> Claims {
        Claims {
            user,
            exp: (chrono::Utc::now().timestamp() + 3600 * 4) as usize, // 4 hour expiration
        }
    }
}

#[allow(dead_code)]
fn get_validation() -> Validation {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.leeway = 0;
    validation.algorithms = vec![Algorithm::ES256];
    validation
}

#[allow(dead_code)]
pub fn validate(jwt: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    
    let validation = get_validation();
    
    let public_key_pem = fs::read("/shared/ecc_public_key.pem")
        .expect("ecc_public_key.pem not found");

    let decoding_key = match DecodingKey::from_ec_pem(&public_key_pem){
        Ok(key) => key,
        Err(e) => panic!("{}", e),
    };

    let decode = decode::<Claims>(jwt, &decoding_key, &validation);
    
    decode
}
