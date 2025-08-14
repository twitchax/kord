// Shared UI components for Kord Web
use klib::core::{
    base::{HasDescription, HasName, HasPreciseName},
    chord::{Chord, HasChord, HasScale},
};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

// Nav.

/// Navigation link used in the navbar
#[component]
pub fn NavLink(href: &'static str, #[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "relative px-3 py-1.5 rounded-md text-sm text-emerald-100/90 hover:text-white hover:bg-white/5 focus:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400/60 focus-visible:ring-offset-2 focus-visible:ring-offset-emerald-900 transition-colors duration-200 after:absolute after:left-3 after:-bottom-0.5 after:h-0.5 after:w-0 after:bg-emerald-300 after:transition-all after:duration-300 hover:after:w-[calc(100%-1.5rem)]";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    let navigate = use_navigate();
    let to = href;
    view! { <button type="button" class=cls on:click=move |_| { navigate(to, Default::default()); }>{children()}</button> }
}

// Typography.

/// Section wrapper with H2
#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="docs-section">
            <h2 class="text-3xl font-semibold text-sage-800 mb-6">{title}</h2>
            {children()}
        </div>
    }
}

/// H3 subheading
#[component]
pub fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h3 class="text-xl font-semibold text-sage-700 mb-3">{text}</h3> }
}

/// H4 heading used in examples blocks
#[component]
pub fn TertiaryHeading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h4 class="text-lg font-medium text-sage-700 mb-2">{text}</h4> }
}

/// Page title (H1) used on main/home
#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! { <h1 class="text-2xl font-semibold tracking-tight">{children()}</h1> }
}

// Content blocks.

/// Code block wrapper
#[component]
pub fn CodeBlock(#[prop(into)] code: String, #[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let base = "bg-sage-100 p-4 rounded-lg border border-sage-200";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <pre class=cls><code>{code}</code></pre> }
}

/// Card-styled external link
#[component]
pub fn CardLink(#[prop(into)] href: String, #[prop(into)] title: String, #[prop(into)] desc: String) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
        >
            <h3 class="text-lg font-semibold text-sage-800 mb-2">{title}</h3>
            <p class="text-sage-600 text-sm">{desc}</p>
        </a>
    }
}

/// Highlighted output/callout
#[component]
pub fn Callout(children: Children) -> impl IntoView {
    view! { <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400"><code class="text-sage-700">{children()}</code></div> }
}

/// Pale panel with title
#[component]
pub fn Panel(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="bg-sage-50 p-4 rounded-lg border border-sage-200">
            <h2 class="font-semibold text-sage-800 mb-2">{title}</h2>
            {children()}
        </div>
    }
}

/// Shared analysis / result output wrapper. If a chord is provided, renders its details. Always renders panel so it can wrap arbitrary children.
#[component]
pub fn ChordAnalysis(#[prop(optional)] chord: Option<Chord>) -> impl IntoView {
    let chord_section = chord.map(|c| {
        let precise = c.precise_name();
        let description = c.description().to_string();
        let scale = c.scale().into_iter().map(|n| n.name()).collect::<Vec<_>>().join(", ");
        let chord_tones = c.chord().into_iter().map(|n| n.name()).collect::<Vec<_>>().join(", ");

        view! {
            <Panel title=precise>
                <div class="text-sage-700 text-sm leading-relaxed">{description}</div>
                <div class="text-sm"><span class="font-medium">"Scale: "</span>{scale}</div>
                <div class="text-sm"><span class="font-medium">"Chord: "</span>{chord_tones}</div>
            </Panel>
        }
    });

    view! {
        <div class="mt-4">
            {chord_section}
        </div>
    }
}

// Buttons

/// Primary button
#[component]
pub fn PrimaryButton<F>(#[prop(optional, into)] id: Option<String>, #[prop(optional, into)] class: Option<String>, on_click: F, children: Children) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let base = "px-3 py-1.5 rounded bg-indigo-600 text-white hover:bg-indigo-500 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400/60 transition-colors";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <button id=id class=cls on:click=on_click>{children()}</button> }
}

/// Secondary button
#[component]
pub fn SecondaryButton<F>(#[prop(optional, into)] class: Option<String>, on_click: F, children: Children) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let base = "px-3 py-1.5 rounded bg-slate-200 hover:bg-slate-300 text-slate-900 transition-colors";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <button class=cls on:click=on_click>{children()}</button> }
}

// Other.

// Small pill badge.
#[component]
pub fn Badge(#[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium select-none";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <span class=cls>{children()}</span> }
}
