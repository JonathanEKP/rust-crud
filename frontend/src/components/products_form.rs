use crate::{
    api::api_create_product,
    store::{set_product, set_show_alert, Store},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn ProductForm() -> Html{

    let (store, dispatch) = use_store::<Store>();
    let name = use_state(String::new);
    let price = use_state(String::new);
    let quantity = use_state(String::new);
    
    let name_input_ref = use_node_ref();
    let price_input_ref = use_node_ref();
    let quantity_input_ref = use_node_ref();

    let handle_name_input = {
        let name = name.clone();
        Callback::from(move |event: InputEvent|{
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            name.set(value);
        })
    };

    let handle_price_input ={
        let price = price.clone();
        Callback::from(move |event: InputEvent|{
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            price.set(value);
        })
    };

    let handle_quantity_input ={
        let quantity = quantity.clone();
        Callback::from(move |event: InputEvent|{
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            quantity.set(value);
        })
    };


    let on_submit = {
        let cloned_dispatch = dispatch.clone();
        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_price_input_ref = price_input_ref.clone();
        let cloned_quantity_input_ref = quantity_input_ref.clone();
        let cloned_name = name.clone();
        let cloned_price = price.clone();
        let cloned_quantity = quantity.clone();

        Callback::from(move |event: SubmitEvent|{
            let name_input_ref = cloned_name_input_ref.clone();
            let price_input_ref = cloned_price_input_ref.clone();
            let quantity_input_ref = cloned_quantity_input_ref.clone();
            let dispatch = cloned_dispatch.clone();
            let name = cloned_name.clone();
            let price = cloned_price.clone();
            let quantity = cloned_quantity.clone();

            event.prevent_default();

            let product_data = serde_json::json!({
                "product_name": name.to_string(),
                "price": price.to_string(),
                "quantity": quantity.to_string(),
            });

            spawn_local(async move {
                let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                let price_input = price_input_ref.cast::<HtmlInputElement>().unwrap();
                let quantity_input = quantity_input_ref.cast::<HtmlInputElement>().unwrap();
                name_input.set_value("");
                price_input.set_value("");
                quantity_input.set_value("");

                name.set(String::new());
                price.set(String::new());
                quantity.set(String::new());

                let response = api_create_product(product_data.to_string()).await;
                
                match response {
                    Ok(product) => {
                        set_show_alert("Producto agregado correctamente".to_string(), dispatch.clone());
                        set_product(product, dispatch);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
            
        })
    };

    html! {
        <form onsubmit={on_submit} class="w-50 bg-secondary-subtle rounded-3 p-3" id="form">
            <h1 class="text-center text-uppercase mb-3">{"Agrega un nuevo producto"}</h1>
            <div class="mb-3">
                <label for="name" class="form-label">{"Nombre del producto"}</label>
                <input type="text" class="form-control" id="name" required=true ref={name_input_ref} oninput={handle_name_input}  />
            </div>
            <div class="mb-3">
                <label for="price" class="form-label">{"Precio del producto"}</label>
                <input type="number" class="form-control" id="price" required=true min="1" pattern="^[0-9]+" ref={price_input_ref} oninput={handle_price_input} />
            </div>
            <div class="mb-3">
                <label for="quantity" class="form-label">{"Cantidad"}</label>
                <input type="number" class="form-control" id="quantity" required=true min="1" pattern="^[0-9]+" ref={quantity_input_ref} oninput={handle_quantity_input} />
            </div>
            <button type="submit" class="btn btn-success" >{"Agregar Producto"}</button>
        </form>
    }
}