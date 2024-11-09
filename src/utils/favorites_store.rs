use tokio::fs::{File, OpenOptions, create_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::path::Path;
use std::error::Error as StdError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::info;

use crate::models::product::Product;
use crate::utils::error::CustomError;

#[derive(Serialize, Deserialize, Clone)]
pub struct Favorites {
    pub user_id: Uuid,
    pub items: Vec<Product>,
}

pub struct FavoritesStore {
    pub favorites: Mutex<Vec<Favorites>>,
    pub favorites_file_path: String,
    pub products_file_path: String,
}

impl FavoritesStore {
    pub async fn new(favorites_file_path: String, products_file_path: String) -> Result<Self, Box<dyn StdError>> {
        let path = Path::new(&favorites_file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.expect("Failed to create directories for favorites.json file");
        }

        if !path.exists() {
            let mut file = File::create(path).await.expect("Failed to create favorites.json file");
            file.write_all(b"[]").await.expect("Failed to write empty array to file");
        }

        let file = File::open(path).await.expect("Failed to open favorites.json file");
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data).await.expect("Failed to read file");

        let favorites: Vec<Favorites> = serde_json::from_str(&data)?;

        Ok(FavoritesStore {
            favorites: Mutex::new(favorites),
            favorites_file_path,
            products_file_path,
        })
    }

    pub async fn save(&self) -> Result<(), Box<dyn StdError>> {
        let favorites = self.favorites.lock().await;
        let data = serde_json::to_string_pretty(&*favorites)?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.favorites_file_path)
            .await
            .expect("Failed to open favorites.json file for writing");

        file.write_all(data.as_bytes()).await?;
        info!("Favorites successfully saved.");
        Ok(())
    }

    async fn load_product_by_id(&self, product_id: Uuid) -> Result<Product, Box<dyn StdError>> {
        let mut file = match File::open(&self.products_file_path).await {
            Ok(f) => f,
            Err(_) => {
                return Err(Box::new(CustomError {
                    message: "Product file not found".to_string(),
                    error_code: "PRODUCT_FILE_NOT_FOUND".to_string(),
                }));
            }
        };

        let mut data = String::new();
        file.read_to_string(&mut data).await?;

        let products: Vec<Product> = serde_json::from_str(&data).unwrap_or_default();
        products.into_iter().find(|p| p.uuid == product_id)
            .ok_or_else(|| "Product not found".into())
    }

    pub async fn add_product_to_favorites(&self, user_id: Uuid, product_id: Uuid) -> Result<(), Box<dyn StdError>> {
        let product = self.load_product_by_id(product_id).await?;

        let mut favorites = self.favorites.lock().await;

        let user_favorites = favorites.iter_mut().find(|f| f.user_id == user_id);
        match user_favorites {
            Some(f) => {
                if !f.items.iter().any(|p| p.uuid == product.uuid) {
                    f.items.push(product);
                } else {
                    return Err(Box::new(CustomError {
                        message: "Product already in favorites".to_string(),
                        error_code: "PRODUCT_ALREADY_IN_FAVORITES".to_string(),
                    }));
                }
            }
            None => {
                let new_favorites = Favorites {
                    user_id,
                    items: vec![product],
                };
                favorites.push(new_favorites);
            }
        }

        drop(favorites);
        self.save().await?;
        Ok(())
    }

    pub async fn get_favorites(&self, user_id: Uuid) -> Vec<Product> {
        let favorites = self.favorites.lock().await;
        favorites.iter().find(|f| f.user_id == user_id).map(|f| f.items.clone()).unwrap_or_default()
    }

    pub async fn remove_product_from_favorites(&self, user_id: Uuid, product_id: Uuid) -> Result<(), Box<dyn StdError>> {
        let mut favorites = self.favorites.lock().await;

        if let Some(favorites_list) = favorites.iter_mut().find(|f| f.user_id == user_id) {
            let initial_len = favorites_list.items.len();
            favorites_list.items.retain(|p| p.uuid != product_id);

            if favorites_list.items.len() < initial_len {
                drop(favorites);
                self.save().await?;
                Ok(())
            } else {
                Err(Box::new(CustomError {
                    message: "Product not found in favorites".to_string(),
                    error_code: "PRODUCT_NOT_FOUND_IN_FAVORITES".to_string(),
                }))
            }
        } else {
            Err(Box::new(CustomError {
                message: "User favorites not found".to_string(),
                error_code: "USER_FAVORITES_NOT_FOUND".to_string(),
            }))
        }
    }
}
