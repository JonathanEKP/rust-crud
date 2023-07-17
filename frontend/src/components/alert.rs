use crate::store::{set_hide_alert, Store};
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub message: String,
    pub delay_ms: u32,
}

#[function_component]
pub fn AlertComponent(props: &Props) -> Html {
     let (store, dispatch) = use_store::<Store>();
     let show_alert = store.alert_input.show_alert;

     use_effect_with_deps(
         move |(show_alert, dispatch, delay_ms)| {
             let cloned_dispatch = dispatch.clone();
             if *show_alert {
                 let handle =
                     Timeout::new(*delay_ms, move || set_hide_alert(cloned_dispatch)).forget();
                 let clear_handle = move || {
                     web_sys::Window::clear_timeout_with_handle(
                         &web_sys::window().unwrap(),
                         handle.as_f64().unwrap() as i32,
                     );
                 };

                 Box::new(clear_handle) as Box<dyn FnOnce()>
             } else {
                 Box::new(|| {}) as Box<dyn FnOnce()>
             }
         },
         (show_alert, dispatch.clone(), props.delay_ms),
     );

    html! {
    <div id="myToast" class={format!("fixed-top px-5 py-4 rounded-3 border border-danger bg- shadow-sm {}", if show_alert { "" } else { "visually-hidden" })}>
        <p class="fs-5">
            <span class="mr-2 inline-block px-3 py-1 text-white fw-bold bg-primary rounded-5">{"i"}</span>
            {props.message.clone()}
        </p>
    </div>
    }
}
