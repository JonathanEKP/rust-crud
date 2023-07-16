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
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductSchema{
    pub name: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
}