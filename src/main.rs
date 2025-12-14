use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger;
use sqlx::mysql::MySqlPool;


mod admin;
mod json;
mod routes;
mod jwt;
mod structs;
mod enums;
mod impl_user;
mod env;
mod views;
mod parse_multipart;

use routes::register_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("database url should be setted");
    
    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let json_conf = json::json_config();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_conf.clone())
            .service(Files::new("/uploads/profile_pictures", "./uploads/profile_pictures").index_file("404"))
            .service(Files::new("/uploads/files", "./uploads/files").index_file("404"))
            .configure(register_services)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
