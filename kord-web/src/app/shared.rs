// Shared UI components for Kord Web
use klib::core::{
    base::{HasDescription, HasName, HasPreciseName},
    chord::{Chord, HasChord, HasScale},
};
use leptos::ev::MouseEvent;
use leptos::{logging::error, prelude::*};
use leptos_router::hooks::use_navigate;
use thaw::{Button, ButtonAppearance, Text, TextTag};
use thaw_utils::BoxOneCallback;

use crate::ffi::highlight_code_block;

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

/// Page title (H1) used on main/home
#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! { <Text tag=TextTag::H1 class="kord-page-title">{children()}</Text> }
}

/// Section wrapper with H2
#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="kord-section">
            <Text tag=TextTag::H2 class="kord-section__title">{title}</Text>
            {children()}
        </div>
    }
}

/// H3 subheading
#[component]
pub fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! { <Text tag=TextTag::H3 class="kord-subheading">{text}</Text> }
}

/// H4 heading used in examples blocks
#[component]
pub fn TertiaryHeading(#[prop(into)] text: String) -> impl IntoView {
    view! { <Text tag=TextTag::H4 class="kord-tertiary-heading">{text}</Text> }
}

// Content blocks.

/// Code block wrapper
#[component]
pub fn CodeBlock(#[prop(into)] code: String, #[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let code_block = NodeRef::new();

    let base = "kord-code-block";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());

    Effect::new(move |_| {
        highlight_code_block(&code_block).unwrap_or_else(|e| error!("Highlight error: {e}"));
    });

    view! { <pre class=cls><code node_ref=code_block>{code}</code></pre> }
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
            <Text tag=TextTag::H2 class="kord-panel__title">{title}</Text>
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
pub fn PrimaryButton<OC>(#[prop(optional, into)] id: Option<String>, #[prop(optional, into)] class: Option<String>, on_click: OC, children: Children) -> impl IntoView
where
    OC: Into<BoxOneCallback<MouseEvent>>,
{
    // These base classes are for selection purposes only (no style at the moment).
    let base = "kord-button kord-button--primary";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());

    view! { <Button attr:id=id appearance=ButtonAppearance::Primary class=cls on_click=on_click.into()>{children()}</Button> }
}

/// Secondary button
#[component]
pub fn SecondaryButton<OC>(#[prop(optional, into)] class: Option<String>, on_click: OC, children: Children) -> impl IntoView
where
    OC: Into<BoxOneCallback<MouseEvent>>,
{
    // These base classes are for selection purposes only (no style at the moment).
    let base = "kord-button kord-button--secondary";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());

    view! {
        <Button
            class=cls
            appearance=ButtonAppearance::Secondary
            on_click=on_click.into()
        >{children()}</Button>
    }
}

// Other.

/// Small pill badge.
#[component]
pub fn Badge(#[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "kord-badge";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    view! { <span class=cls>{children()}</span> }
}
