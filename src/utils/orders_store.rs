use chrono::Utc;
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::path::Path;
use tokio::fs::{create_dir_all, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::product::Product;
use crate::state::app_state::AppState;
use crate::utils::cart_store::ProductWithCount;
use crate::utils::error::CustomError;

#[derive(Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Received,
    Canceled,
    InTransit,
    Returned,
    AtCustoms,
    DisputeOpen,
    DisputeClosed,
    PreparingForShipment,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<Product>,
    pub total_price: f64,
    pub created_at: String,
    pub discount: Option<f64>,
    pub promo_code: Option<String>,
    pub delivery_address: String,
    pub payment_card_number: String,
    pub order_status: OrderStatus,
}

pub struct OrdersStore {
    pub orders: Mutex<Vec<Order>>,
    pub orders_file_path: String,
}

impl OrdersStore {
    pub async fn new(orders_file_path: String) -> Result<Self, Box<dyn StdError>> {
        let path = Path::new(&orders_file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.expect("Failed to create directories for orders.json file");
        }

        if !path.exists() {
            let mut file = File::create(path).await.expect("Failed to create orders.json file");
            file.write_all(b"[]").await.expect("Failed to write empty array to file");
        }

        let file = File::open(path).await.expect("Failed to open orders.json file");
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data).await.expect("Failed to read file");

        let orders: Vec<Order> = serde_json::from_str(&data)?;

        Ok(OrdersStore {
            orders: Mutex::new(orders),
            orders_file_path,
        })
    }

    pub async fn save(&self) -> Result<(), Box<dyn StdError>> {
        let orders = self.orders.lock().await;
        let data = serde_json::to_string_pretty(&*orders)?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.orders_file_path)
            .await
            .expect("Failed to open orders.json file for writing");

        file.write_all(data.as_bytes()).await?;
        info!("Orders successfully saved.");
        Ok(())
    }

    pub async fn add_order(
        &self,
        user_id: Uuid,
        selected_product_ids: Vec<Uuid>,
        promo_code: Option<String>,
        app_state: &AppState
    ) -> Result<(), Box<dyn StdError>> {
        let user = app_state.users_store.find_user_by_id(user_id).await
            .ok_or_else(|| {
                Box::new(CustomError::new("User not found", "USER_NOT_FOUND"))
            })?;

        let payment_card_number = if let Some(credit_cards) = &user.credit_cards {
            if let Some(primary_card) = credit_cards.iter().find(|card| card.is_primary) {
                primary_card.card_number.clone()
            } else {
                return Err(Box::new(CustomError {
                    message: "Primary credit card not found.".to_string(),
                    error_code: "PRIMARY_CREDIT_CARD_NOT_FOUND".to_string(),
                }));
            }
        } else {
            return Err(Box::new(CustomError {
                message: "No credit cards found for user.".to_string(),
                error_code: "CREDIT_CARDS_NOT_FOUND".to_string(),
            }));
        };

        let delivery_address = format!(
            "{}{}, {}, {}, {}, {}, {}",
            user.address.clone().unwrap_or_default(),
            user.city.clone().unwrap_or_default(),
            user.region.clone().unwrap_or_default(),
            user.country.clone().unwrap_or_default(),
            user.zip_code.clone().unwrap_or_default(),
            user.phone_number.clone().unwrap_or_default(),
            user.username
        );

        let cart_items = app_state.carts_store.get_cart(user_id).await;

        let selected_items: Vec<ProductWithCount> = cart_items
            .into_iter()
            .filter(|item| selected_product_ids.contains(&item.product.uuid))
            .collect();

        if selected_items.is_empty() {
            return Err(Box::new(CustomError {
                message: "No selected products found in the cart.".to_string(),
                error_code: "SELECTED_PRODUCTS_NOT_FOUND".to_string(),
            }));
        }

        let mut total_price: f64 = 0.0;
        let mut total_discount: f64 = 0.0;

        for item in &selected_items {
            let clean_price = item.product.price
                .replace(",", "")
                .replace(".", "")
                .replace(" ", "")
                .trim()
                .to_string();

            let mut price: f64 = clean_price.parse().map_err(|e| {
                format!("Failed to parse price for product {}: {}", item.product.uuid, e)
            })?;

            if let Some(discount) = item.product.discount {
                let discount_amount = price * (discount / 100.0);
                price -= discount_amount;
                total_discount += discount_amount * item.count as f64;
            }

            total_price += price * item.count as f64;
        }

        if let Some(code) = promo_code.clone() {
            let (new_total_price, promo_discount) = app_state.promocodes_store.apply_promo_code(&code, total_price).await?;
            total_price = new_total_price;
            total_discount += promo_discount;
        }

        if total_price < 0.0 {
            total_price = 0.0;
        }

        let order = Order {
            order_id: Uuid::new_v4(),
            user_id,
            items: selected_items.into_iter().map(|item| item.product).collect(),
            total_price,
            created_at: Utc::now().to_rfc3339(),
            discount: Some(total_discount),
            promo_code,
            delivery_address,
            payment_card_number,
            order_status: OrderStatus::Received,
        };

        let mut orders = self.orders.lock().await;
        orders.push(order);
        drop(orders);

        self.save().await?;

        app_state.carts_store.remove_products_from_cart(user_id, selected_product_ids).await?;

        Ok(())
    }

    pub async fn get_orders(&self, user_id: Uuid) -> Vec<Order> {
        let orders = self.orders.lock().await;
        orders.iter().filter(|o| o.user_id == user_id).cloned().collect()
    }

    pub async fn remove_order(&self, user_id: Uuid, order_id: Uuid) -> Result<(), Box<dyn StdError>> {
        let mut orders = self.orders.lock().await;

        if let Some(pos) = orders.iter().position(|o| o.user_id == user_id && o.order_id == order_id) {
            orders.remove(pos);
            drop(orders);
            self.save().await?;
            Ok(())
        } else {
            Err(Box::new(CustomError {
                message: "Order not found".to_string(),
                error_code: "ORDER_NOT_FOUND".to_string(),
            }))
        }
    }
}
