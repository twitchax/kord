#![allow(dead_code)]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use kord_web::app::{describe::DescribePage, docs::DocsPage, guess::GuessPage, home::HomePage, listen::ListenPage};
use leptos::{prelude::*, task::tick};
use leptos_router::components::Router;

/// Test that the home page renders without errors
#[wasm_bindgen_test]
async fn test_home_page_loads() {
    mount_to_body(|| {
        view! {
            <Router>
                <HomePage/>
            </Router>
        }
    });

    tick().await;

    let document = document();

    // Check that the hero section renders
    let hero = document.query_selector(".kord-home__hero");
    assert!(hero.is_ok(), "Hero section should be queryable");
    assert!(hero.unwrap().is_some(), "Hero section should exist");
}

/// Test that the docs page renders without errors
#[wasm_bindgen_test]
async fn test_docs_page_loads() {
    mount_to_body(DocsPage);

    tick().await;

    let document = document();

    // Check that the page title renders
    let title = document.query_selector("h1");
    assert!(title.is_ok(), "Title should be queryable");
    assert!(title.unwrap().is_some(), "Title should exist");
}

/// Test that the describe page renders without errors
#[wasm_bindgen_test]
async fn test_describe_page_loads() {
    mount_to_body(DescribePage);

    tick().await;

    let document = document();

    // Check that the input field is present (interactive feature)
    let input = document.query_selector("input");
    assert!(input.is_ok(), "Input should be queryable");
    assert!(input.unwrap().is_some(), "Input field should exist");
}

/// Test that the guess page renders without errors
#[wasm_bindgen_test]
async fn test_guess_page_loads() {
    mount_to_body(GuessPage);

    tick().await;

    let document = document();

    // Check that the input field is present (interactive feature)
    let input = document.query_selector("input");
    assert!(input.is_ok(), "Input should be queryable");
    assert!(input.unwrap().is_some(), "Input field should exist");
}

/// Test that the listen page renders without errors
#[wasm_bindgen_test]
async fn test_listen_page_loads() {
    mount_to_body(ListenPage);

    tick().await;

    let document = document();

    // Check that the chord input is present (interactive feature)
    let input = document.query_selector("input");
    assert!(input.is_ok(), "Input should be queryable");
    assert!(input.unwrap().is_some(), "Input field should exist");
}
