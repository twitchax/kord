use std::collections::HashMap;

use crate::app::shared::NavLink;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::{ssr::SSRMountStyleProvider, ConfigProvider, Theme};
pub mod describe;
pub mod docs;
pub mod guess;
pub mod home;
pub mod listen;
pub mod piano;
pub mod shared;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>

                    // Google Fonts for Inter typeface.
                    <link rel="preconnect" href="https://fonts.googleapis.com" />
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet" />
                    <link href="https://cdn.rawgit.com/Killercodes/281792c423a4fe5544d9a8d36a4430f2/raw/36c2eb3e0c44133880485a143717bda9d180f2c1/GistDarkCode.css" rel="stylesheet" type="text/css" />

                    // Highlight.js for syntax highlighting.
                    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/github-dark-dimmed.min.css" />
                    <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></Script>
                    <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/rust.min.js"></Script>

                    // Soundfont player for MIDI playback.
                    <Script src="https://unpkg.com/soundfont-player@0.12.0/dist/soundfont-player.js"></Script>

                    // Leptos auto-reload, hydration, and meta tags.
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <MetaTags/>
                </head>
                <body>
                    <App/>
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Brand config.

    let brand_colors = HashMap::from([
        (10, "#0f1917"),
        (20, "#132421"),
        (30, "#163129"),
        (40, "#1b3c34"),
        (50, "#23493b"),
        (60, "#2b5a49"),
        (70, "#31705d"),
        (80, "#3b8170"),
        (90, "#4a9b7d"),
        (100, "#5bb88f"),
        (110, "#71d9a2"),
        (120, "#8beec0"),
        (130, "#a6f3d0"),
        (140, "#c6fff0"),
        (150, "#eafff3"),
        (160, "#f7fff9"),
    ]);

    // Theme config.

    let theme = RwSignal::new(Theme::custom_dark(&brand_colors));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kord-web.css"/>

        // sets the document title
        <Title text="Kord"/>

        // Router and layout
        <ConfigProvider theme>
            <Router>
                <NavBar/>
                <main class="kord-main">
                    <div class="kord-content">
                        <div class="kord-content__spacer-lg">
                            <Routes fallback=|| view! { <p class="kord-error">"Page not found."</p> }>
                                <Route path=StaticSegment("") view=home::HomePage/>
                                <Route path=StaticSegment("docs") view=docs::DocsPage/>
                                <Route path=StaticSegment("describe") view=describe::DescribePage/>
                                <Route path=StaticSegment("guess") view=guess::GuessPage/>
                                <Route path=StaticSegment("listen") view=listen::ListenPage/>
                            </Routes>
                        </div>
                    </div>
                </main>
            </Router>
        </ConfigProvider>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="kord-navbar">
            <div class="kord-navbar__container">
                <div class="kord-navbar__brand">
                    <div class="kord-navbar__icon">"♪"</div>
                    <strong class="kord-navbar__title">"Kord"</strong>
                </div>
                <div class="kord-navbar__links">
                    <NavLink href="/">"Home"</NavLink>
                    <NavLink href="/describe">"Describe"</NavLink>
                    <NavLink href="/guess">"Guess"</NavLink>
                    <NavLink href="/listen">"Listen"</NavLink>
                    <NavLink href="/docs">"Docs"</NavLink>
                </div>
            </div>
        </nav>
    }
}
