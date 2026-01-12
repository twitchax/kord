// Shared UI components for Kord Web
use klib::core::{
    base::{HasDescription, HasName, HasPreciseName},
    chord::{Chord, HasChord, HasScale},
    note::Note,
    pitch::Pitch,
};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Button, ButtonAppearance, Flex, FlexAlign, FlexJustify, Text, TextTag};
use thaw_utils::BoxOneCallback;

use crate::client::{audio::play_chord, ffi::highlight_code_block};

// Nav.

/// Navigation link used in the navbar
#[component]
pub fn NavLink(href: &'static str, #[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "kord-nav-link";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    let navigate = use_navigate();

    view! { <button type="button" class=cls on:click=move |_| { navigate(href, Default::default()); }>{children()}</button> }
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
        // Swallow any errors, since effects sometimes run when DOM is not ready, but runs again when ready.
        let _ = highlight_code_block(&code_block);
    });

    view! { <pre class=cls><code node_ref=code_block>{move || code.clone()}</code></pre> }
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
        let scale = c.scale().into_iter().map(|n| view! { <code style="margin-right: 3px;">{n.name()}</code> }).collect::<Vec<_>>();
        let chord_tones = c.chord().into_iter().map(|n| view! { <code style="margin-right: 3px;">{n.name()}</code> }).collect::<Vec<_>>();

        view! {
            <Panel title=precise>
                <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Start>
                    <div>
                        <div class="kord-chord-analysis__description">{description}</div>
                        <div class="kord-chord-analysis__detail"><span class="kord-chord-analysis__label">Scale: </span>{scale}</div>
                        <div class="kord-chord-analysis__detail"><span class="kord-chord-analysis__label">Chord: </span>{chord_tones}</div>
                    </div>
                    <PrimaryButton on_click=move |_| play_chord(&c, 3.0)>"Play"</PrimaryButton>
                </Flex>
            </Panel>
        }
    });

    view! {
        <div class="kord-chord-analysis">
            {chord_section}
        </div>
    }
}

/// Note display component that shows a single note
#[component]
pub fn NoteDisplay(note: Note) -> impl IntoView {
    view! {
        <div class="kord-note-display">
            <code class="kord-note-display__name">{note.name()}</code>
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

// Pitch Visualizer

/// Individual pitch bar item showing a single pitch class with its delta value.
#[component]
fn PitchBarItem<F>(pitch_idx: usize, pitch_deltas: ReadSignal<[f32; 12]>, detected_pitches: ReadSignal<Vec<Pitch>>, active_pitches: ReadSignal<Vec<Pitch>>, on_toggle: F) -> impl IntoView
where
    F: Fn(Pitch) + Copy + Send + 'static,
{
    let pitch_names = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
    let pitch = Pitch::try_from(pitch_idx as u8).unwrap();

    let delta = Signal::derive(move || pitch_deltas.get()[pitch_idx]);
    let is_detected = Signal::derive(move || detected_pitches.with(|p| p.contains(&pitch)));
    let is_active = Signal::derive(move || active_pitches.with(|p| p.contains(&pitch)));

    let bar_height = Signal::derive(move || {
        let d = delta.get();
        // Normalize to 0-100 range, clamping to reasonable bounds
        ((d + 0.5).clamp(0.0, 1.0) * 100.0) as u32
    });

    let class = Signal::derive(move || {
        let mut cls = String::from("kord-pitch-bar");
        if is_active.get() {
            cls.push_str(" kord-pitch-bar--active");
        } else if is_detected.get() {
            cls.push_str(" kord-pitch-bar--detected");
        }
        cls
    });

    view! {
        <div class="kord-pitch-visualizer__item">
            <button
                class=class
                on:click=move |_| on_toggle(pitch)
                title=move || format!(
                    "{}: {:.3} (click to toggle)",
                    pitch_names[pitch_idx],
                    delta.get()
                )
            >
                <div
                    class="kord-pitch-bar__fill"
                    style:height=move || format!("{}%", bar_height.get())
                />
            </button>
            <span class="kord-pitch-visualizer__label">{pitch_names[pitch_idx]}</span>
            <span class="kord-pitch-visualizer__delta">{move || format!("{:.2}", delta.get())}</span>
        </div>
    }
}

/// Interactive pitch visualizer showing delta bars for all 12 pitch classes.
/// Users can click bars to toggle pitches on/off.
#[component]
pub fn PitchVisualizer<F>(pitch_deltas: ReadSignal<[f32; 12]>, detected_pitches: ReadSignal<Vec<Pitch>>, active_pitches: ReadSignal<Vec<Pitch>>, on_toggle: F) -> impl IntoView
where
    F: Fn(Pitch) + Copy + Send + 'static,
{
    view! {
        <div class="kord-pitch-visualizer">
            <For
                each=move || 0..12
                key=|&i| i
                let:pitch_idx
            >
                <PitchBarItem
                    pitch_idx=pitch_idx
                    pitch_deltas=pitch_deltas
                    detected_pitches=detected_pitches
                    active_pitches=active_pitches
                    on_toggle=on_toggle
                />
            </For>
        </div>
    }
}
