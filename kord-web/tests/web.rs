#![allow(dead_code)]

// use wasm_bindgen::JsCast;
// use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);
// use kord_web::app::home::HomePage;
// use leptos::web_sys::HtmlButtonElement;
// use leptos::{prelude::*, task::tick};

// #[wasm_bindgen_test]
// async fn test_integration_home() {
//     mount_to_body(HomePage);

//     let document = document();

//     tick().await;

//     // Get all buttons from the document.
//     let click_me_button = document
//         .query_selector("#click-me")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<HtmlButtonElement>()
//         .expect("Failed to cast to HtmlButtonElement");

//     assert_eq!(click_me_button.inner_text(), "Click Me: 0");

//     // add 3 counters
//     click_me_button.click();
//     click_me_button.click();
//     click_me_button.click();

//     tick().await;

//     // check HTML
//     assert_eq!(click_me_button.inner_text(), "Click Me: 3");
// }
