//ESTAS ESTRUCTURAS DESCRIBEN LA ESTRUCTURA DEL PRODUCTO QUE SQLX VA A RETORNAR DE LA BASE DE DATOS Y LAS RESPUESTAS DEL JSON QUE LA API PROVEERA.
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone )]
pub struct Product{
    pub id: uuid::Uuid,
    pub name: String,
    pub price: String,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductData{
    pub product: Product,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductResponse{
    pub status: String,
    pub data: ProductData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductListResponse{
    pub status: String,
    pub result: i32,
    pub product: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse{
    pub status: String,
    pub message: String,
}