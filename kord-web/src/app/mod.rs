use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

mod about;
mod docs;
mod home;

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
        <Title text="Kord"/>

        // Router and layout
        <Router>
            <NavBar/>
            <main class="max-w-5xl mx-auto p-4">
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
        <nav class="w-full bg-sage-800 text-white">
            <div class="max-w-5xl mx-auto flex items-center gap-4 px-4 py-3">
                <strong class="mr-2 text-sage-100 font-semibold">"♪ Kord"</strong>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/">"Home"</A>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/about">"About"</A>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/docs">"Docs"</A>
            </div>
        </nav>
    }
}
