use tokio::fs::{File, OpenOptions, create_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::path::Path;
use std::error::Error as StdError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use log::{info, error};

use crate::models::product::Product;
use crate::utils::error::CustomError;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductWithCount {
    pub(crate) product: Product,
    pub(crate) count: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cart {
    pub user_id: Uuid,
    pub items: Vec<ProductWithCount>,
}

pub struct CartStore {
    pub carts: Mutex<Vec<Cart>>,
    pub carts_file_path: String,
    pub products_file_path: String,
}

impl CartStore {
    pub async fn new(carts_file_path: String, products_file_path: String) -> Result<Self, Box<dyn StdError>> {
        let path = Path::new(&carts_file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.expect("Failed to create directories for carts.json file");
        }

        if !path.exists() {
            let mut file = File::create(path).await.expect("Failed to create carts.json file");
            file.write_all(b"[]").await.expect("Failed to write empty array to file");
        }

        let file = File::open(path).await.expect("Failed to open carts.json file");
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data).await.expect("Failed to read file");

        let carts: Vec<Cart> = serde_json::from_str(&data)?;

        Ok(CartStore {
            carts: Mutex::new(carts),
            carts_file_path,
            products_file_path,
        })
    }

    pub async fn save(&self) -> Result<(), Box<dyn StdError>> {
        let carts = self.carts.lock().await;
        let data = serde_json::to_string_pretty(&*carts)?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.carts_file_path)
            .await
            .expect("Failed to open carts.json file for writing");

        file.write_all(data.as_bytes()).await?;
        info!("Carts successfully saved.");
        Ok(())
    }

    async fn load_product_by_id(&self, product_id: Uuid) -> Result<Product, Box<CustomError>> {
        let mut file = match File::open(&self.products_file_path).await {
            Ok(f) => {
                info!("Product file successfully opened");
                f
            },
            Err(e) => {
                error!("Failed to open product file: {}", e);
                return Err(Box::new(CustomError {
                    message: "Product file not found".to_string(),
                    error_code: "PRODUCT_FILE_NOT_FOUND".to_string(),
                }));
            }
        };

        let mut data = String::new();
        match file.read_to_string(&mut data).await {
            Ok(_) => info!("Product data successfully read"),
            Err(e) => {
                error!("Error reading product file: {}", e);
                return Err(Box::new(CustomError {
                    message: "Error reading product file".to_string(),
                    error_code: "READ_ERROR".to_string(),
                }));
            }
        }

        let products: Vec<Product> = match serde_json::from_str(&data) {
            Ok(p) => {
                info!("Product JSON successfully deserialized");
                p
            },
            Err(e) => {
                error!("Error deserializing product JSON: {}", e);
                return Err(Box::new(CustomError {
                    message: "Error deserializing product JSON".to_string(),
                    error_code: "DESERIALIZATION_ERROR".to_string(),
                }));
            }
        };

        products.into_iter().find(|p| p.uuid == product_id)
            .ok_or_else(|| {
                error!("Product with ID {} not found", product_id);
                Box::new(CustomError {
                    message: "Product not found".to_string(),
                    error_code: "PRODUCT_NOT_FOUND".to_string(),
                })
            })
    }

    pub async fn add_product_to_cart(&self, user_id: Uuid, product_id: Uuid) -> Result<(), Box<dyn StdError>> {
        let product = self.load_product_by_id(product_id).await?;

        let mut carts = self.carts.lock().await;

        let cart = carts.iter_mut().find(|c| c.user_id == user_id);
        match cart {
            Some(c) => {
                if let Some(item) = c.items.iter_mut().find(|p| p.product.uuid == product.uuid) {
                    item.count += 1;
                } else {
                    c.items.push(ProductWithCount {
                        product,
                        count: 1,
                    });
                }
            }
            None => {
                let new_cart = Cart {
                    user_id,
                    items: vec![ProductWithCount {
                        product,
                        count: 1,
                    }],
                };

                carts.push(new_cart);
            }
        }

        drop(carts);
        self.save().await?;
        Ok(())
    }

    pub async fn get_cart(&self, user_id: Uuid) -> Vec<ProductWithCount> {
        let carts = self.carts.lock().await;
        carts.iter().find(|c| c.user_id == user_id).map(|c| c.items.clone()).unwrap_or_default()
    }

    pub async fn remove_product_from_cart(&self, user_id: Uuid, product_id: Uuid) -> Result<(), Box<dyn StdError>> {
        let mut carts = self.carts.lock().await;

        if let Some(cart) = carts.iter_mut().find(|c| c.user_id == user_id) {
            if let Some(item) = cart.items.iter_mut().find(|p| p.product.uuid == product_id) {
                if item.count > 1 {
                    item.count -= 1;
                } else {
                    cart.items.retain(|p| p.product.uuid != product_id);
                }

                drop(carts);
                self.save().await?;
                Ok(())
            } else {
                return Err(Box::new(CustomError {
                    message: "Product not found in cart".to_string(),
                    error_code: "PRODUCT_NOT_IN_CART".to_string(),
                }));
            }
        } else {
            return Err(Box::new(CustomError {
                message: "User cart not found".to_string(),
                error_code: "USER_CART_NOT_FOUND".to_string(),
            }));
        }
    }

    pub async fn remove_products_from_cart(&self, user_id: Uuid, product_ids: Vec<Uuid>) -> Result<(), Box<dyn StdError>> {
        let mut carts = self.carts.lock().await;

        if let Some(cart) = carts.iter_mut().find(|c| c.user_id == user_id) {
            cart.items.retain(|item| !product_ids.contains(&item.product.uuid));
            drop(carts);
            self.save().await?;
            Ok(())
        } else {
            Err(Box::new(CustomError {
                message: "User cart not found".to_string(),
                error_code: "USER_CART_NOT_FOUND".to_string(),
            }))
        }
    }
}
