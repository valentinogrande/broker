use actix_web::{HttpResponse, Responder, cookie::Cookie, post, web};
use sqlx::MySqlPool;
use bcrypt::verify;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::fs;

use crate::{jwt::Claims, structs::{Creds, User}};


#[post("/login")]
pub async fn login(pool: web::Data<MySqlPool>, creds: web::Json<Creds>) -> impl Responder {
    let result: (u64,String) = match sqlx::query_as("SELECT id, password FROM users WHERE email = ?")
        .bind(&creds.email)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(record) => record,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    }; 

    let hashed_pass = result.1;
    let valid = verify(&creds.password, &hashed_pass).unwrap_or(false);

    if !valid {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    let user_id = result.0;

    
    let claims = Claims::new(User::new(user_id as u64));

    let private_key_pem = match fs::read("/shared/ecc_private_key.pem") {
        Ok(k) => k,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let encoding_key = match EncodingKey::from_ec_pem(&private_key_pem) {
        Ok(k) => k,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let token = match encode(
        &Header::new(Algorithm::ES256),
        &claims,
        &encoding_key,
    ) {
        Ok(t) => t,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let cookie = Cookie::build("jwt", token)
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();

    HttpResponse::Ok().cookie(cookie).json("login success")
}

// login via username should be implemented
