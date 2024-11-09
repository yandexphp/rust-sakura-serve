use actix_web::web;

use crate::controllers::carts_controller::{get_cart, add_product_to_cart, remove_product_from_cart};
use crate::controllers::favorites_controller::{get_favorites, add_product_to_favorites, remove_product_from_favorites};
use crate::controllers::orders_controller::{get_orders, create_order, delete_order};
use crate::controllers::promocodes_controller::{validate_promo_code};

pub fn init_store_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/store")
            .service(
                web::scope("/carts")
                    .service(get_cart)
                    .service(add_product_to_cart)
                    .service(remove_product_from_cart)
            )
            .service(
                web::scope("/favorites")
                    .service(get_favorites)
                    .service(add_product_to_favorites)
                    .service(remove_product_from_favorites)
            )
            .service(
                web::scope("/orders")
                    .service(get_orders)
                    .service(create_order)
                    .service(delete_order)
            )
            .service(
                web::scope("/promocode")
                    .service(validate_promo_code)
            )
    );
}