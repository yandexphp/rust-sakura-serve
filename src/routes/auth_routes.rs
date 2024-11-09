use actix_web::web;

use crate::controllers::auth_controller::{sign_in, logout, sign_up};

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(sign_up)
            .service(sign_in)
            .service(logout)
    );
}
