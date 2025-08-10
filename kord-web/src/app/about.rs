use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-semibold tracking-tight">"About Kord"</h1>
        <p class="mt-3 text-slate-700">
            "Kord is a music theory library and CLI/web app with ML-powered inference."
        </p>
    }
}
