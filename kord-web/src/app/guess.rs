use crate::app::shared::{ChordAnalysis, PageTitle, PrimaryButton};
use leptos::prelude::*;

#[component]
pub fn GuessPage() -> impl IntoView {
    let notes_input = RwSignal::new(String::new());
    let guesses = RwSignal::new(Vec::<String>::new());

    let on_guess = move |_| {
        let raw = notes_input.get();
        let tokens: Vec<_> = raw.split_whitespace().collect();
        if tokens.is_empty() {
            guesses.set(vec![]);
            return;
        }
        // Placeholder dummy guesses
        guesses.set(vec![
            format!("(placeholder) Guess 1 for {:?}", tokens),
            format!("(placeholder) Guess 2 for {:?}", tokens),
            format!("(placeholder) Guess 3 for {:?}", tokens),
        ]);
    };

    view! {
        <PageTitle>"Guess Chords from Notes"</PageTitle>
        <div class="mt-4 flex gap-3 items-end">
            <div class="flex flex-col flex-1 gap-1">
                <label for="guess-notes" class="text-sm font-medium text-sage-700">"Notes"</label>
                <input
                    id="guess-notes"
                    class="w-full px-3 py-2 rounded border border-sage-300 focus:outline-none focus:ring-2 focus:ring-emerald-400 bg-white"
                    placeholder="e.g. C E G Bb"
                    prop:value=move || notes_input.get()
                    on:input=move |ev| notes_input.set(event_target_value(&ev))
                />
            </div>
            <PrimaryButton on_click=on_guess>"Guess"</PrimaryButton>
        </div>
        {move || if guesses.get().is_empty() { None } else { Some(view! {
            <ChordAnalysis></ChordAnalysis>
        })}}
    }
}
