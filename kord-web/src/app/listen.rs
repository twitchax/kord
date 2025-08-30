use crate::{
    app::shared::{ChordAnalysis, PageTitle},
    client::{
        audio::{infer_chords_from_samples, le_bytes_to_f32_samples},
        ffi::record_microphone,
    },
};
use klib::core::chord::Chord;
use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_timestamp;
use thaw::{Button, ButtonAppearance, Flex, FlexGap, FlexJustify, Input, InputSuffix, InputType, ProgressCircle};

#[component]
pub fn ListenPage() -> impl IntoView {
    // Signals.

    let seconds_text = RwSignal::new("10".to_string());
    let seconds = Signal::derive(move || seconds_text.get().parse::<u32>().unwrap_or(10));

    let recording = RwSignal::new(false);

    let error = RwSignal::new(Option::<String>::None);
    let chords = RwSignal::new(Vec::<Chord>::new());

    let timestamp = use_timestamp();
    let start_time = RwSignal::new(None::<f64>);
    let progress_percent = RwSignal::new(0.0);

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

            match infer_chords_from_samples(&samples, secs as u8) {
                Ok(candidates) => {
                    chords.set(candidates);
                }
                Err(e) => {
                    log!("Inference failed: {e}");
                    error.set(Some(e));
                    chords.set(vec![]);
                }
            }

            recording.set(false);
        });
    };

    view! {
    <PageTitle>"Listen"</PageTitle>
    <Flex gap=FlexGap::Large class="kord-content__section">
            <Flex vertical=true gap=FlexGap::Large>
                <Input input_type=InputType::Number value=seconds_text>
                    <InputSuffix slot>
                        "seconds"
                    </InputSuffix>
                </Input>
                <Button
                    disabled=recording
                    appearance=Signal::derive(move || if recording.get() { ButtonAppearance::Subtle } else { ButtonAppearance::Primary })
                    on_click=start
                >{move || if recording.get() { "Recording..." } else { "Start" }}</Button>
                {move || error.get().map(|e| view!{ <p class="kord-error">{e}</p> })}
            </Flex>
            <Flex justify=FlexJustify::Center gap=FlexGap::Small>
                <ProgressCircle value=progress_percent />
            </Flex>
        </Flex>

    <div class="kord-content__section">
            {move || chords.get().into_iter().map(|c| view! { <ChordAnalysis chord=c /> }).collect_view()}
        </div>

    }
}
