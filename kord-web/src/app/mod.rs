use crate::app::shared::NavLink;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub mod describe;
pub mod docs;
pub mod guess;
pub mod home;
pub mod listen;
pub mod shared;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script src="https://cdn.tailwindcss.com"></script>
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet" />
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
            <main class="kord-main">
                <div class="kord-content">
                    <Routes fallback=|| view! { <p class="kord-error">Page not found.</p> }>
                        <Route path=StaticSegment("") view=home::HomePage/>
                        <Route path=StaticSegment("docs") view=docs::DocsPage/>
                        <Route path=StaticSegment("describe") view=describe::DescribePage/>
                        <Route path=StaticSegment("guess") view=guess::GuessPage/>
                        <Route path=StaticSegment("listen") view=listen::ListenPage/>
                    </Routes>
                </div>
            </main>
        </Router>
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
