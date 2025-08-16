use crate::app::shared::PageTitle;
use crate::mic::record_microphone;
use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_timestamp;
use thaw::ProgressCircle;

#[component]
pub fn ListenPage() -> impl IntoView {
    // Signals.

    let seconds = RwSignal::new(10u32);
    let recording = RwSignal::new(false);

    let error = RwSignal::new(Option::<String>::None);

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
            match record_microphone(secs).await {
                Ok(bytes) => {
                    log!("Recorded {} bytes ({} seconds)", bytes.len(), secs);
                }
                Err(e) => {
                    error!("Mic error: {e}");
                    error.set(Some(e));
                }
            }
            recording.set(false);
        });
    };

    view! {
        <PageTitle>"Listen"</PageTitle>
        <div class="flex flex-row gap-4 mt-4 max-w-sm">
            <div class="flex flex-col gap-4 mt-4 w-full">
                <label class="text-sm font-medium">"Seconds"</label>
                <input
                    type="number"
                    min="1"
                    max="120"
                    class="border rounded px-2 py-1"
                    prop:value=move || seconds.get().to_string()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<u32>() { seconds.set(v.max(1)); }
                    }
                />
                <button
                    class="px-3 py-2 rounded bg-emerald-600 text-white disabled:opacity-50"
                    disabled=recording
                    on:click=start
                >{move || if recording.get() { "Recording..." } else { "Start" }}</button>
                {move || error.get().map(|e| view!{ <p class="text-xs text-red-600">{e}</p> })}
            </div>
            <div class="flex items-center gap-2">
                <ProgressCircle value=progress_percent />
            </div>
        </div>

    }
}
