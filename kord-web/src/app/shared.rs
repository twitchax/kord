use leptos::prelude::*;
use leptos_router::components::A;

// Generic, reusable UI components shared across pages

#[component]
pub fn Badge(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base = "px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium select-none";
    let cls = class
        .map(|c| format!("{base} {c}"))
        .unwrap_or_else(|| base.to_string());
    view! { <span class=cls>{children()}</span> }
}

#[component]
pub fn Section(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="docs-section">
            <h2 class="text-3xl font-semibold text-sage-800 mb-6">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
pub fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h3 class="text-xl font-semibold text-sage-700 mb-3">{text}</h3> }
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let base = "bg-sage-100 p-4 rounded-lg border border-sage-200";
    let cls = class
        .map(|c| format!("{base} {c}"))
        .unwrap_or_else(|| base.to_string());
    view! { <pre class=cls><code>{code}</code></pre> }
}

#[component]
pub fn CardLink(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noreferrer"
            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
        >
            <h3 class="text-lg font-semibold text-sage-800 mb-2">{title}</h3>
            <p class="text-sage-600 text-sm">{desc}</p>
        </a>
    }
}

#[component]
pub fn Callout(children: Children) -> impl IntoView {
    view! { <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400"><code class="text-sage-700">{children()}</code></div> }
}

#[component]
pub fn Panel(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-sage-50 p-4 rounded-lg border border-sage-200">
            <h4 class="font-semibold text-sage-800 mb-2">{title}</h4>
            {children()}
        </div>
    }
}

#[component]
pub fn NavLink(
    href: &'static str,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base = "relative px-3 py-1.5 rounded-md text-sm text-emerald-100/90 hover:text-white hover:bg-white/5 focus:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400/60 focus-visible:ring-offset-2 focus-visible:ring-offset-emerald-900 transition-colors duration-200 after:absolute after:left-3 after:-bottom-0.5 after:h-0.5 after:w-0 after:bg-emerald-300 after:transition-all after:duration-300 hover:after:w-[calc(100%-1.5rem)]";
    let cls = class
        .map(|c| format!("{base} {c}"))
        .unwrap_or_else(|| base.to_string());
    view! { <A href=href attr:class=cls>{children()}</A> }
}
