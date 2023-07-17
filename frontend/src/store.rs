use common::Product;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;


/*
    Ahora, podemos utilizar el Yewdux crate para definir un almacén global que 
    contenga los estados de la aplicación.
*/

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub products: Vec<Product>,
    pub alert_input: AlertInput,
}

pub fn set_product(product: Product, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.products.insert(0, product);
    })
}

pub fn set_product_list(products: Vec<Product>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.products = products;
    })
}

pub fn delete_product(id: uuid::Uuid, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.products.retain(|f| f.id != id);
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}