use klib::core::note::Note;
use leptos::{logging::log, prelude::*, task::spawn_local};
use thaw::Flex;

use super::{
    piano::Piano,
    shared::{PageTitle, PrimaryButton, SecondaryButton},
};
use crate::api::hello;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button.
    let count = RwSignal::new(0);

    let on_click_me = move |_| *count.write() += 1;
    let on_click_hello = move |_| {
        let name = "Leptos".to_string();

        spawn_local(async move {
            match hello(name).await {
                Ok(hello_result) => log!("{hello_result}"),
                Err(e) => log!("{e}"),
            }
        });
    };

    let on_key_press = move |n: Note| {
        let nm = klib::core::base::HasName::name(&n);
        log!("pressed: {}", nm);
    };

    view! {
        <PageTitle>"Welcome to Kord!"</PageTitle>
        <Flex class="kord-home__actions">
            <PrimaryButton id="click-me" on_click=on_click_me>
                "Click Me: " {count}
            </PrimaryButton>
            <SecondaryButton on_click=on_click_hello>
                "Click Me for a server call"
            </SecondaryButton>
        </Flex>

        // Piano preview
        <div class="kord-content__section">
            <Piano on_key_press=on_key_press/>
        </div>
    }
}
