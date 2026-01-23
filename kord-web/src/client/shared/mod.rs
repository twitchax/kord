// Shared UI components for Kord Web
use klib::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName},
    chord::{Chord, HasChord, HasRoot, HasScale},
    known_chord::HasScaleCandidates,
    mode::Mode,
    named_pitch::NamedPitch,
    notation::Notation,
    note::Note,
    octave::Octave,
    pitch::{HasFrequency, Pitch},
    scale::Scale,
};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Button, ButtonAppearance, Flex, FlexAlign, FlexJustify, Text, TextTag};
use thaw_utils::BoxOneCallback;

use crate::client::{audio::play_chord, ffi::highlight_code_block};

// Constants for frequency diagram.

/// Maximum number of frequency bins to display (frequencies 0-4000 Hz with 1 Hz resolution).
pub const FREQUENCY_DIAGRAM_MAX_BINS: usize = 4096;

// Nav.

/// Navigation link used in the navbar
#[component]
pub fn NavLink(href: &'static str, #[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let base = "kord-nav-link";
    let cls = class.map(|c| format!("{base} {c}")).unwrap_or_else(|| base.to_string());
    let navigate = use_navigate();

    view! {
        <button
            type="button"
            class=cls
            on:click=move |_| {
                navigate(href, Default::default());
            }
        >
            {children()}
        </button>
    }
}

// Typography.

/// Page title (H1) used on main/home
#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! {
        <Text tag=TextTag::H1 class="kord-page-title">
            {children()}
        </Text>
    }
}

/// Section wrapper with H2
#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="kord-section">
            <Text tag=TextTag::H2 class="kord-section__title">
                {title}
            </Text>
            {children()}
        </div>
    }
}

/// H3 subheading
#[component]
pub fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <Text tag=TextTag::H3 class="kord-subheading">
            {text}
        </Text>
    }
}

/// H4 heading used in examples blocks
#[component]
pub fn TertiaryHeading(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <Text tag=TextTag::H4 class="kord-tertiary-heading">
            {text}
        </Text>
    }
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

    view! {
        <pre class=cls>
            <code node_ref=code_block>{move || code.clone()}</code>
        </pre>
    }
}

/// Card-styled external link
#[component]
pub fn CardLink(#[prop(into)] href: String, #[prop(into)] title: String, #[prop(into)] desc: String) -> impl IntoView {
    view! {
        <a href=href target="_blank" rel="noopener noreferrer" class="kord-card-link">
            <h3 class="kord-card-link__title">{title}</h3>
            <p class="kord-card-link__desc">{desc}</p>
        </a>
    }
}

/// Highlighted output/callout
#[component]
pub fn Callout(children: Children) -> impl IntoView {
    view! {
        <div class="kord-callout">
            <code class="kord-callout__code">{children()}</code>
        </div>
    }
}

/// Pale panel with title
#[component]
pub fn Panel(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="kord-panel">
            <Text tag=TextTag::H2 class="kord-panel__title">
                {title}
            </Text>
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

        // Get scale/mode candidates for this chord.
        let candidates = c.scale_candidates();
        let root = c.root();
        let candidates_view = if candidates.is_empty() {
            None
        } else {
            Some(view! {
                <div class="kord-chord-analysis__candidates">
                    <h4 class="kord-chord-analysis__candidates-title">
                        "Recommended Scales/Modes"
                    </h4>
                    <ul class="kord-chord-analysis__candidates-list">
                        {candidates
                            .into_iter()
                            .map(|candidate| {
                                let rank = candidate.rank();
                                let name = candidate.name();
                                let reason = candidate.reason();
                                let notes = candidate
                                    .notes(root)
                                    .into_iter()
                                    .map(|n| n.static_name())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                let desc = candidate.description();

                                view! {
                                    <li class="kord-chord-analysis__candidate">
                                        <div class="kord-chord-analysis__candidate-header">
                                            <span class="kord-chord-analysis__candidate-rank">
                                                {rank}
                                            </span>
                                            <span class="kord-chord-analysis__candidate-name">
                                                {name}
                                            </span>
                                        </div>
                                        <div class="kord-chord-analysis__candidate-reason">
                                            {reason}
                                        </div>
                                        <div class="kord-chord-analysis__candidate-notes">
                                            <code>{notes}</code>
                                        </div>
                                        <div class="kord-chord-analysis__candidate-desc">
                                            {desc}
                                        </div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>
            })
        };

        view! {
            <Panel title=precise>
                <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Start>
                    <div>
                        <div class="kord-chord-analysis__description">{description}</div>
                        <div class="kord-chord-analysis__detail">
                            <span class="kord-chord-analysis__label">Scale:</span>
                            {scale}
                        </div>
                        <div class="kord-chord-analysis__detail">
                            <span class="kord-chord-analysis__label">Chord:</span>
                            {chord_tones}
                        </div>
                    </div>
                    <PrimaryButton on_click=move |_| play_chord(&c, 3.0)>"Play"</PrimaryButton>
                </Flex>
                {candidates_view}
            </Panel>
        }
    });

    view! { <div class="kord-chord-analysis">{chord_section}</div> }
}

/// Scale analysis component that shows scale details.
#[component]
pub fn ScaleAnalysis(#[prop(optional)] scale: Option<Scale>) -> impl IntoView {
    let scale_section = scale.map(|s| {
        let name = s.name();
        let description = s.description().to_string();
        let notes = s.notes().into_iter().map(|n| view! { <code style="margin-right: 3px;">{n.name()}</code> }).collect::<Vec<_>>();

        view! {
            <Panel title=name>
                <div>
                    <div class="kord-chord-analysis__description">{description}</div>
                    <div class="kord-chord-analysis__detail">
                        <span class="kord-chord-analysis__label">Notes:</span>
                        {notes}
                    </div>
                </div>
            </Panel>
        }
    });

    view! { <div class="kord-scale-analysis">{scale_section}</div> }
}

/// Mode analysis component that shows mode details.
#[component]
pub fn ModeAnalysis(#[prop(optional)] mode: Option<Mode>) -> impl IntoView {
    let mode_section = mode.map(|m| {
        let name = m.name();
        let description = m.description().to_string();
        let notes = m.notes().into_iter().map(|n| view! { <code style="margin-right: 3px;">{n.name()}</code> }).collect::<Vec<_>>();

        view! {
            <Panel title=name>
                <div>
                    <div class="kord-chord-analysis__description">{description}</div>
                    <div class="kord-chord-analysis__detail">
                        <span class="kord-chord-analysis__label">Notes:</span>
                        {notes}
                    </div>
                </div>
            </Panel>
        }
    });

    view! { <div class="kord-mode-analysis">{mode_section}</div> }
}

/// Unified notation analysis component that renders Chord, Scale, or Mode.
#[component]
pub fn NotationAnalysis(notation: Notation) -> impl IntoView {
    match notation {
        Notation::Chord(chord) => view! { <ChordAnalysis chord=chord /> }.into_any(),
        Notation::Scale(scale) => view! { <ScaleAnalysis scale=scale /> }.into_any(),
        Notation::Mode(mode) => view! { <ModeAnalysis mode=mode /> }.into_any(),
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

    view! {
        <Button attr:id=id appearance=ButtonAppearance::Primary class=cls on_click=on_click.into()>
            {children()}
        </Button>
    }
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
        <Button class=cls appearance=ButtonAppearance::Secondary on_click=on_click.into()>
            {children()}
        </Button>
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

    // Derived display signals.

    let title_text = Signal::derive(move || format!("{}: {:.3} (click to toggle)", pitch_names[pitch_idx], delta.get()));
    let bar_height_style = Signal::derive(move || format!("{}%", bar_height.get()));
    let delta_text = Signal::derive(move || format!("{:.2}", delta.get()));

    view! {
        <div class="kord-pitch-visualizer__item">
            <button class=class on:click=move |_| on_toggle(pitch) title=title_text>
                <div class="kord-pitch-bar__fill" style:height=bar_height_style />
            </button>
            <span class="kord-pitch-visualizer__label">{pitch_names[pitch_idx]}</span>
            <span class="kord-pitch-visualizer__delta">{delta_text}</span>
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
            <For each=move || 0..12 key=|&i| i let:pitch_idx>
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

// Frequency Diagram

/// Octave marker data: (label, frequency in Hz).
fn get_octave_markers() -> Vec<(&'static str, f32)> {
    vec![
        ("C1", Note::new(NamedPitch::C, Octave::One).frequency()),
        ("C2", Note::new(NamedPitch::C, Octave::Two).frequency()),
        ("C3", Note::new(NamedPitch::C, Octave::Three).frequency()),
        ("C4", Note::new(NamedPitch::C, Octave::Four).frequency()),
        ("C5", Note::new(NamedPitch::C, Octave::Five).frequency()),
        ("C6", Note::new(NamedPitch::C, Octave::Six).frequency()),
        ("C7", Note::new(NamedPitch::C, Octave::Seven).frequency()),
        ("C8", Note::new(NamedPitch::C, Octave::Eight).frequency()),
    ]
}

/// Convert a frequency to a logarithmic position (0.0 to 1.0) within the display range.
fn freq_to_log_position(freq: f32, min_freq: f32, max_freq: f32) -> f32 {
    if freq <= min_freq {
        return 0.0;
    }
    if freq >= max_freq {
        return 1.0;
    }

    let log_min = min_freq.ln();
    let log_max = max_freq.ln();
    let log_freq = freq.ln();

    (log_freq - log_min) / (log_max - log_min)
}

/// Frequency diagram component displaying FFT frequency bins as vertical bars on a log scale.
///
/// Renders the frequency space data as a canvas-like visualization using pure CSS/HTML.
/// Each bar represents a frequency bin's magnitude, normalized to the maximum value.
/// Uses logarithmic frequency scaling and shows octave boundary markers.
#[component]
pub fn FrequencyDiagram(
    /// The frequency space data as (frequency, magnitude) pairs.
    frequency_data: ReadSignal<Vec<(f32, f32)>>,
) -> impl IntoView {
    // Frequency range for display (roughly C1 to C8).
    let min_freq = 30.0f32;
    let max_freq = 4200.0f32;

    // Downsample the data using max-pooling into logarithmic bins.
    let bars = Signal::derive(move || {
        let data = frequency_data.get();

        if data.is_empty() {
            return vec![];
        }

        // Filter to frequency range of interest.
        let filtered: Vec<_> = data.iter().filter(|(f, _)| *f >= min_freq && *f <= max_freq).copied().collect();

        if filtered.is_empty() {
            return vec![];
        }

        // Find the max magnitude for normalization.
        let max_magnitude = filtered.iter().map(|(_, m)| *m).fold(0.0f32, f32::max);

        if max_magnitude == 0.0 {
            return vec![];
        }

        // Create logarithmic bins.
        let num_bins = 200;
        let mut bins: Vec<(f32, f32)> = Vec::with_capacity(num_bins);

        for i in 0..num_bins {
            let t0 = i as f32 / num_bins as f32;
            let t1 = (i + 1) as f32 / num_bins as f32;

            // Convert linear position to frequency using inverse log.
            let log_min = min_freq.ln();
            let log_max = max_freq.ln();
            let f0 = (log_min + t0 * (log_max - log_min)).exp();
            let f1 = (log_min + t1 * (log_max - log_min)).exp();

            // Find max magnitude in this frequency range.
            let (best_freq, best_mag) = filtered
                .iter()
                .filter(|(f, _)| *f >= f0 && *f < f1)
                .fold((f0, 0.0f32), |(bf, bm), (f, m)| if *m > bm { (*f, *m) } else { (bf, bm) });

            let normalized = (best_mag / max_magnitude * 100.0).clamp(0.0, 100.0);
            bins.push((best_freq, normalized));
        }

        bins
    });

    // Compute octave marker positions with frequency values.
    let octave_markers = Signal::derive(move || {
        get_octave_markers()
            .into_iter()
            .filter(|(_, freq)| *freq >= min_freq && *freq <= max_freq)
            .map(|(label, freq)| {
                let position = freq_to_log_position(freq, min_freq, max_freq) * 100.0;
                (label, freq, position)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="kord-frequency-diagram">
            <div class="kord-frequency-diagram__container">
                // Octave labels at the top.
                <div class="kord-frequency-diagram__labels kord-frequency-diagram__labels--top">
                    <For each=move || octave_markers.get() key=|(label, _, _)| *label let:marker>
                        {
                            let (label, _, position) = marker;
                            let left_pct = format!("{}%", position);
                            view! {
                                <span class="kord-frequency-diagram__label" style:left=left_pct>{label}</span>
                            }
                        }
                    </For>
                </div>

                // Main bar area with marker lines.
                <div class="kord-frequency-diagram__chart">
                    <div class="kord-frequency-diagram__bars">
                        <For each=move || bars.get().into_iter().enumerate() key=|(i, (_, h))| (*i, (*h * 100.0) as u32) let:entry>
                            {
                                let (_, (freq, height)) = entry;
                                let style = format!("height: {}%;", height);
                                let title = format!("{:.0} Hz", freq);
                                view! { <div class="kord-frequency-diagram__bar" style=style title=title /> }
                            }
                        </For>
                    </div>
                    <div class="kord-frequency-diagram__markers">
                        <For each=move || octave_markers.get() key=|(label, _, _)| *label let:marker>
                            {
                                let (_, _, position) = marker;
                                let left_pct = format!("{}%", position);
                                view! {
                                    <div class="kord-frequency-diagram__marker" style:left=left_pct>
                                        <div class="kord-frequency-diagram__marker-line" />
                                    </div>
                                }
                            }
                        </For>
                    </div>
                </div>

                // Frequency labels at the bottom.
                <div class="kord-frequency-diagram__labels kord-frequency-diagram__labels--bottom">
                    <For each=move || octave_markers.get() key=|(label, _, _)| *label let:marker>
                        {
                            let (_, freq, position) = marker;
                            let left_pct = format!("{}%", position);
                            let freq_label = format!("{:.0}", freq);
                            view! {
                                <span class="kord-frequency-diagram__label" style:left=left_pct>{freq_label}</span>
                            }
                        }
                    </For>
                </div>
            </div>
        </div>
    }
}
