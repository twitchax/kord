// Turn off coverage in the web app, since it is merely the "showcase".
#![feature(coverage_attribute)]
#![coverage(off)]
// Leptos' hydrate view types nest deeply; the wasm hydrate build overflows
// the default 128 query-depth limit on newer nightlies.
#![recursion_limit = "512"]

pub mod api;
pub mod app;
pub mod client;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
