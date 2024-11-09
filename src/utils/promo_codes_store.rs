use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::path::Path;
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};

use crate::utils::error::CustomError;

#[derive(Serialize, Deserialize, Clone)]
pub struct PromoCode {
    pub code: String,
    pub discount: f64,
    pub available_at: String,
    pub expired_at: String,
}

#[derive(Clone)]
pub struct PromoCodesStore {
    pub promocodes_file_path: String,
}

impl PromoCodesStore {
    pub async fn new(promocodes_file_path: String) -> Result<Self, Box<dyn StdError>> {
        let path = Path::new(&promocodes_file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.expect("Failed to create directories for promocodes.json file");
        }

        if !path.exists() {
            let mut file = File::create(path).await.expect("Failed to create promocodes.json file");
            file.write_all(b"[]").await.expect("Failed to write empty array to file");
        }

        let file = File::open(path).await.expect("Failed to open promocodes.json file");
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data).await.expect("Failed to read file");

        let _: Vec<PromoCode> = serde_json::from_str(&data)?;

        Ok(PromoCodesStore { promocodes_file_path })
    }

    pub async fn load_promo_codes(&self) -> Result<Vec<PromoCode>, Box<dyn StdError>> {
        let mut file = File::open(&self.promocodes_file_path).await?;
        let mut data = String::new();
        file.read_to_string(&mut data).await?;

        let promo_codes: Vec<PromoCode> = serde_json::from_str(&data)?;
        Ok(promo_codes)
    }

    pub async fn get_promo_codes(&self) -> Result<Vec<PromoCode>, Box<dyn StdError>> {
        let promo_codes = self.load_promo_codes().await?;

        if promo_codes.is_empty() {
            Err(Box::new(CustomError {
                message: "No promo codes available".to_string(),
                error_code: "NO_PROMO_CODES_FOUND".to_string(),
            }))
        } else {
            Ok(promo_codes)
        }
    }

    pub async fn get_promo_code(&self, code: &str) -> Result<PromoCode, Box<dyn StdError>> {
        let promo_codes = self.get_promo_codes().await?;

        if let Some(promo) = promo_codes.into_iter().find(|p| p.code == code) {
            Ok(promo)
        } else {
            Err(Box::new(CustomError {
                message: "Promo code not found".to_string(),
                error_code: "PROMO_CODE_NOT_FOUND".to_string(),
            }))
        }
    }

    pub async fn apply_promo_code(&self, code: &str, total_price: f64) -> Result<(f64, f64), Box<dyn StdError>> {
        let promo = self.get_promo_code(code).await?;

        let current_time = Utc::now();
        let available_at = promo.available_at.parse::<chrono::DateTime<Utc>>().unwrap();
        let expired_at = promo.expired_at.parse::<chrono::DateTime<Utc>>().unwrap();

        if current_time < available_at {
            return Err(Box::new(CustomError {
                message: "Promo code is not yet available".to_string(),
                error_code: "PROMO_CODE_NOT_YET_AVAILABLE".to_string(),
            }));
        } else if current_time > expired_at {
            return Err(Box::new(CustomError {
                message: "Promo code has expired".to_string(),
                error_code: "PROMO_CODE_EXPIRED".to_string(),
            }));
        }

        let discount = total_price * (promo.discount / 100.0);
        let new_price = (total_price - discount).max(0.0);
        Ok((new_price, discount))
    }
}
