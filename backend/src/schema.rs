use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProductSchema{
    pub product_name: String,
    pub price: String,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductSchema{
    pub product_name: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
}