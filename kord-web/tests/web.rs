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

    // Check that the badge renders
    let badge = document.query_selector(".kord-home__badge");
    assert!(badge.is_ok(), "Badge should be queryable");
    assert!(badge.unwrap().is_some(), "Badge should exist");

    // Check that CTA buttons render
    let cta_buttons = document.query_selector_all(".kord-home__cta-button");
    assert!(cta_buttons.is_ok(), "CTA buttons should be queryable");
    assert_eq!(cta_buttons.unwrap().length(), 2, "Should have 2 CTA buttons");

    // Check that feature grid renders
    let feature_grid = document.query_selector(".kord-home__feature-grid");
    assert!(feature_grid.is_ok(), "Feature grid should be queryable");
    assert!(feature_grid.unwrap().is_some(), "Feature grid should exist");

    // Check that all 4 features render
    let features = document.query_selector_all(".kord-home__feature");
    assert!(features.is_ok(), "Features should be queryable");
    assert_eq!(features.unwrap().length(), 4, "Should have 4 feature cards");
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

    // Check that the docs container renders
    let docs = document.query_selector(".kord-docs");
    assert!(docs.is_ok(), "Docs container should be queryable");
    assert!(docs.unwrap().is_some(), "Docs container should exist");

    // Check that badges render (at least one)
    let badges = document.query_selector_all(".kord-badge");
    assert!(badges.is_ok(), "Badges should be queryable");
    assert!(badges.unwrap().length() > 0, "Should have at least one badge");

    // Check that code blocks render (installation examples)
    let code_blocks = document.query_selector_all(".kord-code-block");
    assert!(code_blocks.is_ok(), "Code blocks should be queryable");
    assert!(code_blocks.unwrap().length() > 0, "Should have at least one code block");
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

    // Check that the page title renders
    let title = document.query_selector("h1");
    assert!(title.is_ok(), "Title should be queryable");
    assert!(title.unwrap().is_some(), "Title should exist");

    // Check that the field label renders
    let field = document.query_selector(".thaw-field");
    assert!(field.is_ok(), "Field container should be queryable");
    assert!(field.unwrap().is_some(), "Field container should exist");
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

    // Check that the page title renders
    let title = document.query_selector("h1");
    assert!(title.is_ok(), "Title should be queryable");
    assert!(title.unwrap().is_some(), "Title should exist");

    // Check that the piano component renders
    let piano = document.query_selector(".kord-piano");
    assert!(piano.is_ok(), "Piano should be queryable");
    assert!(piano.unwrap().is_some(), "Piano component should exist");

    // Check that piano keys render
    let piano_keys = document.query_selector_all(".kord-piano__key");
    assert!(piano_keys.is_ok(), "Piano keys should be queryable");
    assert!(piano_keys.unwrap().length() > 0, "Should have piano keys");
}

/// Test that the listen page renders without errors
#[wasm_bindgen_test]
async fn test_listen_page_loads() {
    mount_to_body(ListenPage);

    tick().await;

    let document = document();

    // Check that the seconds input is present
    let input = document.query_selector("input");
    assert!(input.is_ok(), "Input should be queryable");
    assert!(input.unwrap().is_some(), "Input field should exist");

    // Check that the page title renders
    let title = document.query_selector("h1");
    assert!(title.is_ok(), "Title should be queryable");
    assert!(title.unwrap().is_some(), "Title should exist");

    // Check that the record button renders
    let button = document.query_selector("button");
    assert!(button.is_ok(), "Button should be queryable");
    assert!(button.unwrap().is_some(), "Record button should exist");

    // Check that the progress circle renders
    let progress = document.query_selector(".thaw-progress-circle");
    assert!(progress.is_ok(), "Progress circle should be queryable");
    assert!(progress.unwrap().is_some(), "Progress circle should exist");

    // Check that result sections render (even if empty initially)
    let results = document.query_selector_all(".kord-listen__results");
    assert!(results.is_ok(), "Results sections should be queryable");
    assert_eq!(results.unwrap().length(), 3, "Should have 3 results sections (pitches, notes, chords)");
}
