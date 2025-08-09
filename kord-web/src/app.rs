use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

use crate::api::hello;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script src="https://cdn.tailwindcss.com"></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kord-web.css"/>

        // sets the document title
        <Title text="Kord"/>

        // Router and layout
        <Router>
            <NavBar/>
            <main class="max-w-5xl mx-auto p-4">
                <div class="mt-6">
                    <Routes fallback=|| view! { <p class="text-sm text-red-600">"Page not found."</p> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("about") view=AboutPage/>
                        <Route path=StaticSegment("docs") view=DocsPage/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
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

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-semibold tracking-tight">"About Kord"</h1>
        <p class="mt-3 text-slate-700">
            "Kord is a music theory library and CLI/web app with ML-powered inference."
        </p>
    }
}

#[component]
fn DocsPage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-semibold tracking-tight">"Docs"</h1>
        <p class="mt-3 text-slate-700">"Browse the Rust crate documentation:"</p>
        <p class="mt-2">
            <a
                class="text-indigo-600 hover:text-indigo-500 underline"
                href="https://docs.rs/kord/latest/klib/"
                target="_blank"
                rel="noreferrer"
            >
                "kord on docs.rs"
            </a>
        </p>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="w-full bg-slate-800 text-white">
            <div class="max-w-5xl mx-auto flex items-center gap-4 px-4 py-3">
                <strong class="mr-2">"Kord"</strong>
                <A attr:class="text-sm text-white/90 hover:text-white" href="/">"Home"</A>
                <A attr:class="text-sm text-white/90 hover:text-white" href="/about">"About"</A>
                <A attr:class="text-sm text-white/90 hover:text-white" href="/docs">"Docs"</A>
            </div>
        </nav>
    }
}
