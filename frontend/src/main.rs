mod api;
mod components;
mod store;

use components::{
    alert::{AlertComponent, Props as AlertProps},
    product_list::ProductList,
    products_form::ProductForm,
};

use store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html!(
        <>
            <AlertComponent
                message={alert_props.message}
                delay_ms={alert_props.delay_ms}
            />
            // <ProductList/>
            <ProductForm/>

        </>
    )
}

fn main(){
    yew::Renderer::<App>::new().render();
}