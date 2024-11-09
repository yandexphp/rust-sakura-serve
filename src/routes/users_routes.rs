use actix_web::web;

use crate::controllers::users_controller::{get_user, profile, add_credit_card, delete_credit_card, update_profile_full, update_profile_partial};

pub fn init_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(profile)
            .service(get_user)
            .service(add_credit_card)
            .service(delete_credit_card)
            .service(update_profile_full)
            .service(update_profile_partial)
    );
}
