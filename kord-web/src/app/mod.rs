use crate::app::shared::NavLink;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub mod about;
pub mod docs;
pub mod home;
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
            <body style="font-family: Inter, ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, Apple Color Emoji, Segoe UI Emoji;">
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
            <main class="max-w-5xl mx-auto px-4 pb-4 pt-16">
                <div class="mt-6">
                    <Routes fallback=|| view! { <p class="text-sm text-red-600">"Page not found."</p> }>
                        <Route path=StaticSegment("") view=home::HomePage/>
                        <Route path=StaticSegment("about") view=about::AboutPage/>
                        <Route path=StaticSegment("docs") view=docs::DocsPage/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="w-full fixed top-0 left-0 right-0 z-50 bg-gradient-to-r from-emerald-900 to-emerald-800 text-emerald-50 shadow-sm">
            <div class="max-w-5xl mx-auto flex items-center justify-between px-4 py-3">
                <div class="flex items-left gap-3 ml-0 pl-4 select-none">
                    <div class="h-6 w-6 rounded-md bg-emerald-600/80 ring-1 ring-white/10 flex items-center justify-center text-white text-xs font-bold shadow-sm">"♪"</div>
                    <strong class="tracking-tight">"Kord"</strong>
                </div>
                <div class="flex items-center gap-2 mr-0 pr-4">
                    <NavLink href="/">"Home"</NavLink>
                    <NavLink href="/about">"About"</NavLink>
                    <NavLink href="/docs">"Docs"</NavLink>
                </div>
            </div>
        </nav>
    }
}
