use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProductModel{
    pub id: Uuid,
    pub product_name: String,
    pub price: String,
    pub quantity: String,
}