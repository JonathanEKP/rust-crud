use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    api::api_fetch_product,
    store::{set_product_list, set_show_alert, Store}, 
};


#[function_component]
pub fn ProductList() -> Html{
    let (store, dispatch) = use_store::<Store>();
        let product_list = store.products.clone();
    
        use_effect_with_deps(
            move |_| {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = api_fetch_product((1, 10)).await;
                    match response {
                        Ok(product) => {
                            set_product_list(product, dispatch);
                        }
                        Err(e) => {
                            set_show_alert(e.to_string(), dispatch);
                        }
                    }
                });
            },
            (),
        );
        /*
        
                product_list.into_iter().map(|product|{
                    let key = product.id.to_string();
                    html!{<div {key} product={product.clone()}>  </div>}
                }).collect::<Html>()
         */

        

    html! {
        <div>
            {
                product_list.into_iter().map(|product| {
                    html!{<div key={product.id.to_string()}>{ format!("Hello, I'am {}!",product.name) }</div>}
                }).collect::<Html>()
            }

            <h1>{"Productos"}</h1>
        </div>
    }
}