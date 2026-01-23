use crate::client::{
    audio::{chords_from_pitches, compute_frequency_space, infer_from_samples, le_bytes_to_f32_samples, pitches_to_notes},
    ffi::record_microphone,
    shared::{ChordAnalysis, FrequencyDiagram, NoteDisplay, PageTitle, PitchVisualizer, FREQUENCY_DIAGRAM_MAX_BINS},
};
use klib::core::{base::HasName, chord::Chord, note::Note, pitch::Pitch};
use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_timestamp;
use thaw::{Button, ButtonAppearance, Field, Flex, FlexGap, FlexJustify, Input, InputSuffix, InputType, ProgressCircle};

#[component]
pub fn ListenPage() -> impl IntoView {
    // Signals.

    let seconds_text = RwSignal::new("10".to_string());
    let seconds = Signal::derive(move || seconds_text.get().parse::<u32>().unwrap_or(10));

    let recording = RwSignal::new(false);

    let error = RwSignal::new(Option::<String>::None);
    let pitch_deltas = RwSignal::new([0.0f32; 12]);
    let detected_pitches = RwSignal::new(Vec::<Pitch>::new());
    let active_pitches = RwSignal::new(Vec::<Pitch>::new());
    let notes = Signal::derive(move || pitches_to_notes(&active_pitches.get()));
    let chords = RwSignal::new(Vec::<Chord>::new());
    let frequency_data = RwSignal::new(Vec::<(f32, f32)>::new());

    let timestamp = use_timestamp();
    let start_time = RwSignal::new(None::<f64>);
    let progress_percent = RwSignal::new(0.0);

    // Derived visibility signals for Show components.

    let has_detected_pitches = Signal::derive(move || !detected_pitches.with(|p| p.is_empty()));
    let has_notes = Signal::derive(move || !notes.with(|n| n.is_empty()));
    let has_chords = Signal::derive(move || !chords.with(|c| c.is_empty()));
    let has_frequency_data = Signal::derive(move || !frequency_data.with(|d| d.is_empty()));

    // Derived UI state signals.

    let button_appearance = Signal::derive(move || if recording.get() { ButtonAppearance::Subtle } else { ButtonAppearance::Primary });
    let button_text = Signal::derive(move || if recording.get() { "Recording..." } else { "Start Listening" });
    let status_text = Signal::derive(move || if recording.get() { "Listening..." } else { "Idle" });

    // Effect that updates the progress percentage.

    Effect::watch(
        move || (recording.get(), timestamp.get()),
        move |(is_recording, current_timestamp), previous, _| {
            if !*is_recording {
                // If we aren't recording, clear values.
                start_time.set(None);
                progress_percent.set(0.0);

                return;
            }

            let was_recording = previous.map(|(was_recording, _)| was_recording).unwrap_or(&false);

            // If we _just_ started recording, clear values.
            if *is_recording && !*was_recording {
                start_time.set(Some(*current_timestamp));
                progress_percent.set(0.0);
            }

            // If we are recording, update progress.
            if let Some(start) = start_time.get_untracked() {
                let elapsed = current_timestamp - start;
                let total = (seconds.get_untracked() * 1000) as f64;
                let progress = (elapsed / total * 100.0).clamp(0.0, 100.0).round();

                progress_percent.set(progress);
            }
        },
        false,
    );

    // Effect to recompute chords when active pitches change.
    Effect::watch(
        move || active_pitches.get(),
        move |pitches, _, _| match chords_from_pitches(pitches) {
            Ok(candidates) => chords.set(candidates),
            Err(e) => {
                log!("Chord generation failed: {e}");
                chords.set(vec![]);
            }
        },
        false,
    );

    // Pitch toggle handler.
    let toggle_pitch = move |pitch: Pitch| {
        active_pitches.update(|pitches| {
            if let Some(pos) = pitches.iter().position(|&p| p == pitch) {
                pitches.remove(pos);
            } else {
                pitches.push(pitch);
                pitches.sort();
            }
        });
    };

    // Start handler.

    let start = move |_| {
        // Do nothing if already recording.
        if recording.get() {
            return;
        }

        error.set(None);
        recording.set(true);

        let secs = seconds.get();

        spawn_local(async move {
            let bytes = match record_microphone(secs).await {
                Ok(b) => b,
                Err(e) => {
                    error!("Mic error: {e}");
                    error.set(Some(e));
                    recording.set(false);
                    return;
                }
            };

            let samples = match le_bytes_to_f32_samples(&bytes) {
                Ok(s) => s,
                Err(e) => {
                    error!("{e}");
                    error.set(Some(e.to_string()));
                    recording.set(false);
                    return;
                }
            };

            match infer_from_samples(&samples, secs as u8) {
                Ok(result) => {
                    pitch_deltas.set(result.pitch_deltas);
                    detected_pitches.set(result.pitches.clone());
                    active_pitches.set(result.pitches);
                    chords.set(result.chords);

                    // Compute frequency space for visualization.
                    let freq_space = compute_frequency_space(&samples, secs as u8, FREQUENCY_DIAGRAM_MAX_BINS);
                    frequency_data.set(freq_space);
                }
                Err(e) => {
                    log!("Inference failed: {e}");
                    error.set(Some(e.to_string()));
                    pitch_deltas.set([0.0f32; 12]);
                    detected_pitches.set(vec![]);
                    active_pitches.set(vec![]);
                    chords.set(vec![]);
                    frequency_data.set(vec![]);
                }
            }

            recording.set(false);
        });
    };

    view! {
        <PageTitle>"Listen"</PageTitle>
        <section class="kord-listen">
            <Flex gap=FlexGap::Large class="kord-content__section kord-listen__controls">
                <Flex vertical=true gap=FlexGap::Medium class="kord-listen__form">
                    <p class="kord-listen__hint">
                        "Capture a quick snippet to identify the chords in your surroundings."
                    </p>
                    <Field label="Recording length">
                        <Input input_type=InputType::Number value=seconds_text>
                            <InputSuffix slot>"seconds"</InputSuffix>
                        </Input>
                    </Field>
                    <Button
                        class="kord-listen__start"
                        disabled=recording
                        appearance=button_appearance
                        on_click=start
                    >
                        {button_text}
                    </Button>
                    {move || error.get().map(|e| view! { <p class="kord-error">{e}</p> })}
                </Flex>
                <Flex
                    vertical=true
                    justify=FlexJustify::Center
                    gap=FlexGap::Small
                    class="kord-listen__progress"
                >
                    <ProgressCircle value=progress_percent />
                    <span class="kord-listen__status">{status_text}</span>
                </Flex>
            </Flex>

            <div class="kord-content__section kord-listen__results">
                <h3 class="kord-listen__results-title">"Pitch Detection"</h3>
                <Show
                    when=has_detected_pitches
                    fallback=move || {
                        view! {
                            <p class="kord-listen__empty">
                                "Pitch data will appear here after recording."
                            </p>
                        }
                            .into_view()
                    }
                >
                    <PitchVisualizer
                        pitch_deltas=pitch_deltas.read_only()
                        detected_pitches=detected_pitches.read_only()
                        active_pitches=active_pitches.read_only()
                        on_toggle=toggle_pitch
                    />
                </Show>
            </div>

            <div class="kord-content__section kord-listen__results">
                <h3 class="kord-listen__results-title">"Frequency Space"</h3>
                <Show
                    when=has_frequency_data
                    fallback=move || {
                        view! {
                            <p class="kord-listen__empty">
                                "Frequency spectrum will appear here after recording."
                            </p>
                        }
                            .into_view()
                    }
                >
                    <FrequencyDiagram frequency_data=frequency_data.read_only() />
                </Show>
            </div>

            <div class="kord-content__section kord-listen__results">
                <h3 class="kord-listen__results-title">"Detected Notes"</h3>
                <div class="kord-listen__notes">
                    <Show
                        when=has_notes
                        fallback=move || {
                            view! {
                                <p class="kord-listen__empty">
                                    "Press start to analyze live audio."
                                </p>
                            }
                                .into_view()
                        }
                    >
                        <Flex gap=FlexGap::Small class="kord-notes-list">
                            <For
                                each=move || notes.get()
                                key=|note: &Note| note.name()
                                children=move |note: Note| view! { <NoteDisplay note=note /> }
                            />
                        </Flex>
                    </Show>
                </div>
            </div>

            <div class="kord-content__section kord-listen__results">
                <h3 class="kord-listen__results-title">"Detected Chords"</h3>
                <div class="kord-listen__chords">
                    <Show
                        when=has_chords
                        fallback=move || {
                            view! {
                                <p class="kord-listen__empty">
                                    "Notes will be analyzed into chords above."
                                </p>
                            }
                                .into_view()
                        }
                    >
                        <For
                            each=move || chords.get()
                            key=|chord: &Chord| chord.to_string()
                            children=move |chord: Chord| view! { <ChordAnalysis chord=chord /> }
                        />
                    </Show>
                </div>
            </div>
        </section>
    }
}
