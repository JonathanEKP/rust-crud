use common::{ErrorResponse, Product, ProductListResponse, ProductResponse};
use reqwasm::http;


//FUNCION PARA AGREGAR UN PRODUCTO A LA BASE DE DATOS
pub async fn api_create_product(product_data: String) -> Result<Product, String>{
    let response = match http::Request::post("http://localhost:8000/api/products/")
        .header("Content-type", "application/json")
        .body(product_data)
        .send()
        .await
    {
        Ok(res) => {
            println!("Producto agregado con exito");
            res
        },
        Err(_) => return Err("Error al hacer una peticion".to_string()),
    };

    if response.status() != 200{
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response{
            return Err(error_response.message);
        }else{
            return Err(format!("Error en la API: {}", response.status()));
        }
    }
    let res_json = response.json::<ProductResponse>().await;
    match res_json{
        Ok(data) =>Ok(data.data.product),
        Err(_) => Err("Error al convertir la respuesta. Producto agregado correctamente".to_string()),
    }
}

//FUNCION PARA OBTENER LOS PRODUCTOS
pub async fn api_fetch_product((page, limit) : (i32, i32)) -> Result<Vec<Product>, String>{
    let response = match http::Request::get(
        format!(
            "http://localhost:8000/api/products?page={}&limit={}",
            page, limit
        )
        .as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Error al hacer una peticion".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("Error en la API: {}", response.status()));
        }
    }

    let res_json = response.json::<ProductListResponse>().await;
    match res_json{
        Ok(data) => Ok(data.product),
        Err(_) => Err("Error al convertir la respuesta".to_string()),
    }
}

//FUNCION PARA ELIMINAR PRODUCTO
pub async fn api_delete_product(product_id: &str) -> Result<(), String>{
    let response = match http::Request::delete(
        format!("http://localhost:8000/api/products/{}", product_id).as_str(),
    ).send().await
    {
        Ok(res) => res,
        Err(_) => return Err("Error al hacer una peticion".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response{
            return Err(error_response.message);
        }else{
            return Err(format!("Error en la API: {}", response.status()));
        }
    }
    Ok(())
}




//TODO: FUNCION PARA ACTUALIZAR PRODUCTO

//TODO: FUNCION PARA BUSCAR PRODUCTO POR ID
