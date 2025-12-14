use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};

use crate::enums::Subject;
use crate::structs::{NewCompany, NewIndividual, NewUser};
use crate::jwt::validate;

#[post("/register")]
pub async fn register_user(pool: web::Data<MySqlPool>, user: web::Json<NewUser>) -> impl Responder{
    let hashed_pass = match hash(&user.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let _query = match sqlx::query("insert into users (username, email, password) values (?, ?, ?)")
        .bind(&user.username)
        .bind(&user.email)
        .bind(hashed_pass)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

}


#[post("/register/individuals")]
pub async fn register_individual(pool: web::Data<MySqlPool>, req: HttpRequest,individual: web::Json<NewIndividual>) -> impl Responder {

    let cookie = match req.cookie("jwt") {
        Some(cookie) => cookie,
        None => return HttpResponse::Unauthorized().json("Missing JWT cookie"),
    };

    let token = match validate(cookie.value()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid JWT token"),
    };

    let user = token.claims.user;

    let _ = user.create_kyc(&pool, Subject::Individual).await;

    match user.create_individual(&pool, individual.into_inner()).await {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    }

}

#[post("/register/companies")]
pub async fn register_company(pool: web::Data<MySqlPool>, req: HttpRequest, company: web::Json<NewCompany>) -> impl Responder {

    let cookie = match req.cookie("jwt") {
        Some(cookie) => cookie,
        None => return HttpResponse::Unauthorized().json("Missing JWT cookie"),
    };

    let token = match validate(cookie.value()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid JWT token"),
    };

    let user = token.claims.user;
    
    let _ = user.create_kyc(&pool, Subject::Company).await;

    match user.create_company(&pool, company.into_inner()).await {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    }
}

// veryfication email is still needed
