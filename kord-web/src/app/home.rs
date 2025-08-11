use leptos::{logging::log, prelude::*, task::spawn_local};

use super::shared::{PageTitle, PrimaryButton, SecondaryButton};
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

    view! {
        <PageTitle>"Welcome to Leptos!"</PageTitle>
        <div class="mt-4 flex gap-3">
            <PrimaryButton on_click=move |_| on_click_me(())>
                "Click Me: " {count}
            </PrimaryButton>
            <SecondaryButton on_click=move |_| on_click_hello(())>
                "Click Me for a server call"
            </SecondaryButton>
        </div>
    }
}
