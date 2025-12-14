use actix_web::web;

use crate::views::health::health::health;
use crate::views::login::login::login;
use crate::views::register::register::register_user;
use crate::admin::users::get_users;

pub fn register_services(cfg: &mut web::ServiceConfig) {
    cfg.service(health)
    .service(get_users)
    .service(login)
    .service(register_user);
}
