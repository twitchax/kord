use crate::app::shared::{ChordAnalysis, PageTitle, PrimaryButton};
use klib::core::{base::Parsable, chord::Chord};
use leptos::prelude::*;

#[component]
pub fn DescribePage() -> impl IntoView {
    // Placeholder reactive inputs
    let chord_input = RwSignal::new(String::new());
    let result = RwSignal::new(Option::<Chord>::None);

    let on_describe = move |_| {
        // TODO: integrate with core describe logic
        let val = chord_input.get();
        if val.trim().is_empty() {
            result.set(None);
            return;
        }
        match Chord::parse(&val) {
            Ok(c) => result.set(Some(c)),
            Err(_) => result.set(None),
        }
    };

    view! {
        <PageTitle>"Describe a Chord"</PageTitle>
        <div class="mt-4 flex gap-3 items-end">
            <div class="flex flex-col flex-1 gap-1">
                <label for="describe-chord" class="text-sm font-medium text-sage-700">"Chord Symbol"</label>
                <input
                    id="describe-chord"
                    class="w-full px-3 py-2 rounded border border-sage-300 focus:outline-none focus:ring-2 focus:ring-emerald-400 bg-white"
                    placeholder="e.g. Cm7"
                    prop:value=move || chord_input.get()
                    on:input=move |ev| chord_input.set(event_target_value(&ev))
                />
            </div>
            <PrimaryButton on_click=on_describe>"Describe"</PrimaryButton>
        </div>
        {move || result.get().map(|c| view! {
            <ChordAnalysis chord=c></ChordAnalysis>
        })}
    }
}
