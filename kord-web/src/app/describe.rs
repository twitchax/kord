use crate::app::shared::{ChordAnalysis, PageTitle};
use klib::core::{base::Parsable, chord::Chord};
use leptos::prelude::*;

const DEBOUNCE_MS: u64 = 300;

#[component]
pub fn DescribePage() -> impl IntoView {
    let chord_input = RwSignal::new(String::new());
    let chord_result = RwSignal::new(None);

    let mut debounced_parse = {
        leptos::prelude::debounce(std::time::Duration::from_millis(DEBOUNCE_MS), move |val: String| {
            if val.trim().is_empty() {
                chord_result.set(None);
            } else {
                chord_result.set(Chord::parse(&val).ok());
            }
        })
    };

    let input_keyup = move |ev| {
        let val = event_target_value(&ev);
        chord_input.set(val.clone());

        debounced_parse(val);
    };

    view! {
        <PageTitle>"Describe a Chord"</PageTitle>

        <div class="mt-4 flex flex-col gap-2">
            <label for="describe-chord" class="text-sm font-medium text-sage-700">"Chord Symbol"</label>
            <input
                id="describe-chord"
                class="w-full px-3 py-2 rounded border border-sage-300 focus:outline-none focus:ring-2 focus:ring-emerald-400 bg-white"
                placeholder="e.g. Cm7"
                prop:value=move || chord_input.get()
                on:keyup=input_keyup
            />
            <p class="text-xs text-slate-500">"Typing parses after "{DEBOUNCE_MS}"ms of inactivity."</p>
        </div>

        {move || chord_result.get().map(|c| view! {
            <ChordAnalysis chord=c></ChordAnalysis>
        })}
    }
}
