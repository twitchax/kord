pub mod app;

mod shared;

#[cfg(feature = "ssr")]
mod api;

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
