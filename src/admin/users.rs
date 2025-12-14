use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sqlx::MySqlPool;
use crate::{jwt::validate, structs::UserData};

#[get("/admin/users")]
pub async fn get_users(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    
    let cookie = match req.cookie("jwt") {
        Some(cookie) => cookie,
        None => return HttpResponse::Unauthorized().json("Missing JWT cookie"),
    };

    let token = match validate(cookie.value()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid JWT token"),
    };

    let user = token.claims.user;

    let is_admin = match user.is_admin(pool.as_ref()).await {
    Ok(admin) => admin,
    Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if !is_admin {
        return HttpResponse::Unauthorized().finish();
    }

    let users: Vec<UserData> = match sqlx::query_as::<_, UserData>("select * from users")
        .fetch_all(pool.as_ref())
        .await {
        Ok(u) => u,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

    HttpResponse::Ok().json(users)
}


