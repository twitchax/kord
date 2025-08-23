use crate::app::shared::{ChordAnalysis, PageTitle};
use klib::core::{base::Parsable, chord::Chord, note::Note};
use leptos::prelude::*;
use thaw::{Field, FieldValidationState, Flex, FlexGap, Input, InputRule};

const DEBOUNCE_MS: u64 = 300;

#[component]
pub fn GuessPage() -> impl IntoView {
    // Signals
    let notes_input = RwSignal::new(String::new());
    let chords = RwSignal::new(Vec::<Chord>::new());

    // Debounced processing
    let mut debounced_parse = {
        leptos::prelude::debounce(std::time::Duration::from_millis(DEBOUNCE_MS), move |raw: String| {
            let trimmed = raw.trim();

            if trimmed.is_empty() {
                chords.set(vec![]);
                return;
            }

            // Parse tokens into notes; ignore invalids here because InputRule handles inline errors.
            let mut notes: Vec<Note> = Vec::new();
            for tok in trimmed.split_whitespace() {
                if let Ok(n) = Note::parse(tok) {
                    notes.push(n);
                } else {
                    chords.set(vec![]);
                    return;
                }
            }

            match Chord::try_from_notes(&notes) {
                Ok(mut candidates) => {
                    candidates.truncate(8);
                    chords.set(candidates);
                }
                Err(_) => {
                    chords.set(vec![]);
                }
            }
        })
    };

    // Watch input changes and trigger debounced parsing.
    Effect::watch(
        move || notes_input.get(),
        move |val, _, _| {
            debounced_parse(val.clone());
        },
        false,
    );

    // Inline validation rules for the notes input.
    let tokens_valid_rule = InputRule::validator(|v: &String, _| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        for tok in trimmed.split_whitespace() {
            if Note::parse(tok).is_err() {
                return Err(FieldValidationState::Error(format!("Invalid note '{tok}'")));
            }
        }
        Ok(())
    });
    let min_notes_rule = InputRule::validator(|v: &String, _| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        let count = trimmed.split_whitespace().count();
        if count < 2 {
            Err(FieldValidationState::Error("Enter at least 2 notes".to_string()))
        } else {
            Ok(())
        }
    });
    let rules = vec![tokens_valid_rule, min_notes_rule];

    view! {
        <PageTitle>"Guess Chords from Notes"</PageTitle>
        <Flex vertical=true gap=FlexGap::Large>
            <Field label="Notes (space separated)">
                <Input
                    id="guess-notes"
                    placeholder="e.g. C E G Bb"
                    value=notes_input
                    rules=rules
                />
            </Field>
        </Flex>
        <div>
            {move || chords.get().into_iter().map(|c| view! { <ChordAnalysis chord=c /> }).collect_view()}
        </div>
    }
}
