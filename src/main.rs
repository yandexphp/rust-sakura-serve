mod config;
mod controllers;
mod models;
mod routes;
mod utils;
mod state;

use actix_session::{SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use log::info;
use std::sync::Arc;
use std::path::Path;
use tokio::fs;

use crate::utils::user_store::UserStore;
use crate::utils::cart_store::CartStore;
use crate::utils::favorites_store::FavoritesStore;
use crate::utils::orders_store::OrdersStore;
use crate::utils::logger::init_logger;
use crate::state::app_state::AppState;
use crate::utils::promo_codes_store::PromoCodesStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    init_logger();

    let config = config::Config::from_env().expect("Failed to load configuration");

    info!("🚀 サーバーが {} で起動しています...", config.server_address);

    ensure_static_directory_exists().await;

    let users_store = Arc::new(UserStore::new(config.users_file_path.clone())
        .await
        .expect("Failed to initialize UserStore"));
    
    let carts_store = Arc::new(CartStore::new(config.carts_file_path.clone(), config.products_file_path.clone())
        .await
        .expect("Failed to initialize CartStore"));
    
    let favorites_store = Arc::new(FavoritesStore::new(config.favorites_file_path.clone(), config.products_file_path.clone())
        .await
        .expect("Failed to initialize FavoritesStore"));
    
    let orders_store = Arc::new(OrdersStore::new(config.orders_file_path.clone())
        .await
        .expect("Failed to initialize OrdersStore"));

    let promocodes_store = Arc::new(PromoCodesStore::new(config.promocodes_file_path.clone())
        .await
        .expect("Failed to initialize PromoCodesStore"));

    let app_state = web::Data::new(AppState::new(
        users_store,
        orders_store,
        favorites_store,
        carts_store,
        promocodes_store,
    ));

    let server_address_clone = config.server_address.clone();

    actix_web::rt::spawn(async move {
        actix_web::rt::time::sleep(std::time::Duration::from_secs(1)).await;

        let url = format!("http://{}", server_address_clone);

        match webbrowser::open(&url) {
            Ok(_) => info!("Browser successfully opened at {}", url),
            Err(e) => log::error!("Failed to open browser: {}", e),
        }
    });

    let key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    key.clone(),
                )
                .build()
            )
            .app_data(app_state.clone())
            .configure(routes::init_routes)
    })
    .bind(&config.server_address)?
    .run()
    .await
}

async fn ensure_static_directory_exists() {
    let static_dir_path = "app/www/static";
    let static_dir = Path::new(static_dir_path);

    if !static_dir.exists() {
        if let Err(e) = fs::create_dir_all(static_dir).await {
            log::error!("Failed to create directory `{}`: {}", static_dir_path, e);
        } else {
            log::info!("Created directory `{}`", static_dir_path);
        }
    }
}