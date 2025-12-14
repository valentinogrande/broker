use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
        HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}
