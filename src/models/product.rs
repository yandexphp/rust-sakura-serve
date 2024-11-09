use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub uuid: Uuid,
    pub pathurl: String,
    pub article: String,
    pub price: String,
    pub rating: f64,
    pub reviews: f64,
    pub currency: String,
    pub discount: Option<f64>,
    pub is_new: bool,
    pub image: String,
    pub name: String,
    pub brand: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
}
