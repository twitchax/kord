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
        <Title text="Kord - Music Theory Library"/>

        // content for this welcome page
        <Router>
            <nav class="navbar">
                <div class="navbar-brand">
                    <A href="/">"Kord"</A>
                </div>
                <div class="navbar-nav">
                    <A href="/" class:nav-link=true>"Home"</A>
                    <A href="/about" class:nav-link=true>"About"</A>
                    <A href="/contact" class:nav-link=true>"Contact"</A>
                </div>
            </nav>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                </Routes>
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
        <div class="page-content">
            <h1>"Welcome to Kord!"</h1>
            <p>"A music theory library and CLI tool with ML capabilities."</p>
            <div class="demo-section">
                <h2>"Interactive Demo"</h2>
                <button on:click=on_click_me class="demo-button">"Click Me: " {count}</button>
                <button on:click=on_click_hello class="demo-button">"Click Me for a server call"</button>
            </div>
        </div>
    }
}

/// Renders the about page.
#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="page-content">
            <h1>"About Kord"</h1>
            <p>"Kord is a comprehensive music theory library and CLI tool built in Rust with machine learning capabilities."</p>
            
            <h2>"Features"</h2>
            <ul>
                <li>"Music theory primitives (Note, Chord, Modifier)"</li>
                <li>"Audio analysis with FFT/spectrum processing"</li>
                <li>"Machine learning with Burn framework"</li>
                <li>"Multi-platform support (native, WASM, WASI)"</li>
                <li>"CLI tools for music analysis"</li>
            </ul>

            <h2>"Architecture"</h2>
            <p>"Built as a Cargo workspace with feature flags for different deployment targets:"</p>
            <ul>
                <li><code>"cli"</code>" - Command line interface"</li>
                <li><code>"analyze"</code>" - Audio processing capabilities"</li>
                <li><code>"ml"</code>" - Machine learning features"</li>
                <li><code>"wasm/wasi"</code>" - WebAssembly compilation targets"</li>
            </ul>
        </div>
    }
}

/// Renders the contact page.
#[component]
fn ContactPage() -> impl IntoView {
    view! {
        <div class="page-content">
            <h1>"Contact & Resources"</h1>
            
            <h2>"GitHub Repository"</h2>
            <p>"Find the source code, report issues, and contribute:"</p>
            <a href="https://github.com/twitchax/kord" target="_blank" class="link">"github.com/twitchax/kord"</a>

            <h2>"Documentation"</h2>
            <p>"Learn more about using Kord:"</p>
            <ul>
                <li><a href="https://docs.rs/kord" target="_blank" class="link">"API Documentation"</a></li>
                <li>"README.md in the repository"</li>
                <li>"DEVELOPMENT.md for contribution guidelines"</li>
            </ul>

            <h2>"Getting Started"</h2>
            <p>"To start using Kord, you can:"</p>
            <ol>
                <li>"Install via cargo: " <code>"cargo install kord"</code></li>
                <li>"Clone the repository and build from source"</li>
                <li>"Use the WASM version for web applications"</li>
            </ol>
        </div>
    }
}
