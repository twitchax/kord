use leptos::prelude::*;
use crate::app::shared::{PageTitle, PrimaryButton, AnalysisOutput, SecondaryButton};

#[component]
pub fn ListenPage() -> impl IntoView {
    let is_listening = RwSignal::new(false);
    let status = RwSignal::new(String::from("Idle"));
    let detected = RwSignal::new(Vec::<String>::new());

    let start_listening = move |_| {
        if is_listening.get() { return; }
        is_listening.set(true);
        status.set("(placeholder) Listening...".into());
        // TODO: integrate mic capture + ML inference
    };

    let stop_listening = move |_| {
        if !is_listening.get() { return; }
        is_listening.set(false);
        status.set("(placeholder) Stopped.".into());
        detected.set(vec![
            "(placeholder) C".into(),
            "(placeholder) E".into(),
            "(placeholder) G".into(),
        ]);
    };

    view! {
        <PageTitle>"Listen & Detect"</PageTitle>
        <div class="mt-4 flex gap-3">
            <PrimaryButton on_click=start_listening class="disabled:opacity-50" id="listen-start" >"Start"</PrimaryButton>
            <SecondaryButton on_click=stop_listening>"Stop"</SecondaryButton>
        </div>
        <div class="mt-4 text-sm text-sage-700">{move || status.get()}</div>
        {move || if detected.get().is_empty() { None } else { Some(view! { <AnalysisOutput>
            <div class="flex flex-wrap gap-2">{detected.get().into_iter().map(|n| view! { <span class="px-2 py-1 bg-sage-100 rounded text-sage-700 text-sm">{n}</span> }).collect_view()}</div>
        </AnalysisOutput> })}}
    }
}
