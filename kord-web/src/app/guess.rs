use crate::app::shared::{ChordAnalysis, PageTitle};
use klib::core::{base::Parsable, chord::Chord, note::Note, parser::note_str_to_note};
use leptos::prelude::*;

const DEBOUNCE_MS: u64 = 300;

#[component]
pub fn GuessPage() -> impl IntoView {
    // Signals
    let notes_input = RwSignal::new(String::new());
    let chords = RwSignal::new(Vec::<Chord>::new());
    let error = RwSignal::new(Option::<String>::None);

    // Debounced processing
    let mut debounced_parse = {
        leptos::prelude::debounce(std::time::Duration::from_millis(DEBOUNCE_MS), move |raw: String| {
            let trimmed = raw.trim();

            if trimmed.is_empty() {
                chords.set(vec![]);
                error.set(None);
                return;
            }

            let mut notes: Vec<Note> = Vec::new();
            for tok in trimmed.split_whitespace() {
                match Note::parse(tok) {
                    Ok(n) => notes.push(n),
                    Err(e) => {
                        chords.set(vec![]);
                        error.set(Some(format!("Invalid note '{tok}': {e}")));
                        return;
                    }
                }
            }

            match Chord::try_from_notes(&notes) {
                Ok(mut candidates) => {
                    candidates.truncate(8);

                    error.set(None);
                    chords.set(candidates);
                }
                Err(e) => {
                    chords.set(vec![]);
                    error.set(Some(format!("Guess error: {e}")));
                }
            }
        })
    };

    let input_keyup = move |ev| {
        let val = event_target_value(&ev);
        notes_input.set(val.clone());

        debounced_parse(val);
    };

    view! {
        <PageTitle>"Guess Chords from Notes"</PageTitle>
        <div class="mt-4 flex flex-col gap-2">
            <label for="guess-notes" class="text-sm font-medium text-sage-700">"Notes (space separated)"</label>
            <input
                id="guess-notes"
                class="w-full px-3 py-2 rounded border border-sage-300 focus:outline-none focus:ring-2 focus:ring-emerald-400 bg-white"
                placeholder="e.g. C E G Bb"
                prop:value=move || notes_input.get()
                on:keyup=input_keyup
            />
            <p class="text-xs text-slate-500">"Updates after "{DEBOUNCE_MS}"ms pause"</p>
            {move || error.get().map(|e| view! { <p class="text-xs text-red-600">{e}</p> })}
        </div>
        <div class="mt-4 grid gap-4 md:grid-cols-2">
            {move || chords.get().into_iter().map(|c| view! { <ChordAnalysis chord=c /> }).collect_view()}
        </div>
    }
}
