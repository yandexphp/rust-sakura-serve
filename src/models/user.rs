use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCard {
    pub id: Uuid,
    pub cardholder_name: String,
    pub card_number: String,
    pub expiration_date: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub login: String,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<String>,
    pub avatar_url: Option<String>,
    pub registration_date: DateTime<Utc>,
    pub last_login_date: Option<DateTime<Utc>>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub credit_cards: Option<Vec<CreditCard>>,
}

#[derive(Debug, Deserialize)]
pub struct FullProfileUpdate {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub date_of_birth: String,
    pub avatar_url: Option<String>,
    pub country: String,
    pub region: String,
    pub city: String,
    pub address: String,
    pub zip_code: String,
    pub credit_cards: Option<Vec<CreditCard>>,
}

#[derive(Debug, Deserialize)]
pub struct PartialProfileUpdate {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<String>,
    pub avatar_url: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub credit_cards: Option<Vec<CreditCard>>,
}