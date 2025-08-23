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
    let base = "kord-nav-link";
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
        <div class="kord-section">
            <h2 class="kord-section__title">{title}</h2>
            {children()}
        </div>
    }
}

/// H3 subheading
#[component]
pub fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h3 class="kord-subheading">{text}</h3> }
}

/// H4 heading used in examples blocks
#[component]
pub fn TertiaryHeading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h4 class="kord-tertiary-heading">{text}</h4> }
}

/// Page title (H1) used on main/home
#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! { <h1 class="kord-page-title">{children()}</h1> }
}

// Content blocks.

/// Code block wrapper
#[component]
pub fn CodeBlock(#[prop(into)] code: String, #[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let base = "kord-code-block";
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
            class="kord-card-link"
        >
            <h3 class="kord-card-link__title">{title}</h3>
            <p class="kord-card-link__desc">{desc}</p>
        </a>
    }
}

/// Highlighted output/callout
#[component]
pub fn Callout(children: Children) -> impl IntoView {
    view! { <div class="kord-callout"><code class="kord-callout__code">{children()}</code></div> }
}

/// Pale panel with title
#[component]
pub fn Panel(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="kord-panel">
            <h2 class="kord-panel__title">{title}</h2>
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
                <div class="kord-chord-analysis__description">{description}</div>
                <div class="kord-chord-analysis__detail"><span class="kord-chord-analysis__label">Scale: </span>{scale}</div>
                <div class="kord-chord-analysis__detail"><span class="kord-chord-analysis__label">Chord: </span>{chord_tones}</div>
            </Panel>
        }
    });

    view! {
        <div class="kord-chord-analysis">
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
    let base = "kord-button kord-button--primary";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <button id=id class=cls on:click=on_click>{children()}</button> }
}

/// Secondary button
#[component]
pub fn SecondaryButton<F>(#[prop(optional, into)] class: Option<String>, on_click: F, children: Children) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let base = "kord-button kord-button--secondary";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <button class=cls on:click=on_click>{children()}</button> }
}

// Other.

// Small pill badge.
#[component]
pub fn Badge(#[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "kord-badge";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <span class=cls>{children()}</span> }
}
