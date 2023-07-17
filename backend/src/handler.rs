use crate::{
    model::ProductModel,
    schema::{CreateProductSchema, UpdateProductSchema, FilterOptions},
    AppState,
};

use actix_web::{delete,get,patch,post,web,HttpResponse,Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder{
    const MESSAGE: &str = "API para la prueba técnica de Linea Rosa usando Actix";
    HttpResponse::Ok().json(json!({"status": "succes","message": MESSAGE}))
}


//OBTIENE LA LISTA DE PRODUCTOS
#[get("/products")]
pub async fn product_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder{
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    ).fetch_all(&data.db).await;

    if query_result.is_err(){
        let message = "Algo malo paso mientras se hacia fetch a todos los productos";
        return HttpResponse::InternalServerError().json(json!({"status": "error","message": message}));
    }

    let products = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "succes",
        "result": products.len(),
        "products": products
    });
    HttpResponse::Ok().json(json_response)
}

//INSERTA UN PRODUCTO EN LA BASE DE DATOS
#[post("/products/")]
async fn create_products_handler(
    body: web::Json<CreateProductSchema>,
    data: web::Data<AppState>,
) -> impl Responder{
    let query_result = sqlx::query_as!(
        ProductModel,
        "INSERT INTO products (product_name, price, quantity) VALUES ($1, $2, $3) RETURNING *",
        body.product_name.to_string(),
        body.price.to_string(),
        body.quantity.to_string(),
    ).fetch_one(&data.db).await;

    match query_result{
        Ok(product) => {
            let product_response = serde_json::json!({"status":"succes","data": serde_json::json!({
                "product": product
            })});
            return HttpResponse::Ok().json(product_response);
        }
        Err(e) =>{
            return HttpResponse::InternalServerError().json(serde_json::json!({"status":"error","message":format!("{:?}", e)}));
        }
    }
}

//SELECCIONA UN PRODUCTO ESPECIFICO CON LA ID
#[get("products/{id}")]
async fn get_product_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder{
    let product_id = path.into_inner();
    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE id = $1",
        product_id
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status":"succes","data": serde_json::json!({
                "product": product
            })});
            return HttpResponse::Ok().json(product_response);
        }
        Err(_) => {
            let message = format!("El producto con el ID: {} no fue encontrado", product_id);
            return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
        }
    }

}

//ACTUALIZAR PRODUCTOS
#[patch("/products/{id}")]
async fn edit_product_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateProductSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let product_id = path.into_inner();
    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE id = $1",
        product_id
    ).fetch_one(&data.db).await;

    if query_result.is_err(){
        let message = format!("El producto con el ID: {} no fue encontrado", product_id);
        return HttpResponse::NotFound().json(serde_json::json!({"status":"fail","message": message}));
    }

    let product = query_result.unwrap();

    let query_result = sqlx::query_as!(
        ProductModel,
        "UPDATE products SET product_name = $1, price = $2, quantity = $3 WHERE id = $4 RETURNING *",
        body.product_name.to_owned().unwrap_or(product.product_name),
        body.price.to_owned().unwrap_or(product.price),
        body.quantity.to_owned().unwrap_or(product.quantity),
        product_id
    ).fetch_one(&data.db).await;
    
    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status":"succes","data": serde_json::json!({
                "product": product
            })});

            return HttpResponse::Ok().json(product_response);
        }
        Err(err) =>{
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({"status":"error","message":message}));
        }
    }
}

//ELIMINAR PRODUCTO
#[delete("/products/{id}")]
async fn delete_product_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder{
    let product_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM products WHERE id = $1", product_id).execute(&data.db).await.unwrap().rows_affected();

    if rows_affected == 0 {
        let message = format!("El producto con el ID: {} no fue encontrado", product_id);
        return HttpResponse::NotFound().json(json!({"status":"fail","message":message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
    .service(health_checker_handler)
    .service(product_list_handler)
    .service(create_products_handler)
    .service(get_product_handler)
    .service(edit_product_handler)
    .service(delete_product_handler);

    conf.service(scope);
}