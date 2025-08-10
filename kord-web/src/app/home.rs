use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::api::hello;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
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
        <h1 class="text-2xl font-semibold tracking-tight">"Welcome to Leptos!"</h1>
        <div class="mt-4 flex gap-3">
            <button class="px-3 py-1.5 rounded bg-indigo-600 text-white hover:bg-indigo-500" on:click=on_click_me>
                "Click Me: " {count}
            </button>
            <button class="px-3 py-1.5 rounded bg-slate-200 hover:bg-slate-300" on:click=on_click_hello>
                "Click Me for a server call"
            </button>
        </div>
    }
}
