use std::env;
use dotenv::dotenv;

pub struct Config {
    pub server_address: String,
    pub products_file_path: String,
    pub users_file_path: String,
    pub carts_file_path: String,
    pub favorites_file_path: String,
    pub orders_file_path: String,
    pub promocodes_file_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Self {
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            products_file_path: env::var("DATA_PRODUCTS_FILE_PATH").unwrap_or_else(|_| "data/db/products.json".to_string()),
            users_file_path: env::var("DATA_USERS_FILE_PATH").unwrap_or_else(|_| "data/db/users.json".to_string()),
            carts_file_path: env::var("DATA_CARTS_FILE_PATH").unwrap_or_else(|_| "data/db/carts.json".to_string()),
            favorites_file_path: env::var("DATA_FAVORITES_FILE_PATH").unwrap_or_else(|_| "data/db/favorites.json".to_string()),
            orders_file_path: env::var("DATA_ORDERS_FILE_PATH").unwrap_or_else(|_| "data/db/orders.json".to_string()),
            promocodes_file_path: env::var("DATA_PROMOCODES_FILE_PATH").unwrap_or_else(|_| "data/db/promocodes.json".to_string()),
        })
    }
}
