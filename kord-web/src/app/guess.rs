use crate::client::{
    piano::Piano,
    shared::{ChordAnalysis, PageTitle},
};
use klib::core::{
    base::{HasName, Parsable},
    chord::Chord,
    note::Note,
};
use leptos::prelude::*;
use std::time::Duration;
use thaw::{Field, FieldValidationState, Flex, FlexGap, Input, InputRule};

const DEBOUNCE_MS: u64 = 300;
const MAX_CANDIDATES: usize = 8;

#[component]
pub fn GuessPage() -> impl IntoView {
    // Signals
    let notes_input = RwSignal::new(String::new());
    let chords = RwSignal::new(Vec::<Chord>::new());

    // Debounced processing
    let mut debounced_parse = {
        leptos::prelude::debounce(Duration::from_millis(DEBOUNCE_MS), move |raw: String| {
            let trimmed = raw.trim();

            if trimmed.is_empty() {
                chords.set(vec![]);
                return;
            }

            match parse_notes_input(trimmed) {
                Ok(notes) => {
                    if notes.len() < 2 {
                        chords.set(vec![]);
                        return;
                    }

                    match Chord::try_from_notes(&notes) {
                        Ok(mut candidates) => {
                            candidates.truncate(MAX_CANDIDATES);
                            chords.set(candidates);
                        }
                        Err(_) => chords.set(vec![]),
                    }
                }
                Err(_) => chords.set(vec![]),
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
        match parse_notes_input(trimmed) {
            Ok(_) => Ok(()),
            Err(token) => Err(FieldValidationState::Error(format!("Invalid note '{token}'"))),
        }
    });
    let min_notes_rule = InputRule::validator(|v: &String, _| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        let count = tokens_from_input(trimmed).len();
        if count < 2 {
            Err(FieldValidationState::Error("Enter at least 2 notes".to_string()))
        } else {
            Ok(())
        }
    });
    let rules = vec![tokens_valid_rule, min_notes_rule];

    let on_piano_key_press = {
        let notes_input_handle = notes_input;

        move |note: Note| {
            let note_ascii = note.name_ascii();

            notes_input_handle.update(|current| {
                let mut tokens = tokens_from_input(current);
                if !tokens.iter().any(|existing| existing.eq_ignore_ascii_case(&note_ascii)) {
                    tokens.push(note_ascii.clone());
                }
                *current = tokens.join(" ");
            });
        }
    };

    view! {
        <PageTitle>"Guess Chords from Notes"</PageTitle>
    <Flex vertical=true gap=FlexGap::Large class="kord-content__section kord-content__section--narrow">
            <Field label="Notes">
                <Input
                    id="guess-notes"
                    placeholder="e.g. C E G Bb"
                    value=notes_input
                    rules=rules
                />
            </Field>
        </Flex>
        <div class="kord-content__section">
            <Piano on_key_press=on_piano_key_press />
        </div>
        <div>
            {move || chords.get().into_iter().map(|c| view! { <ChordAnalysis chord=c /> }).collect_view()}
        </div>
    }
}

/// Splits the input string into tokens based on whitespace and common delimiters.
fn tokens_from_input(value: &str) -> Vec<String> {
    value
        .split(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | '|'))
        .filter_map(|token| {
            let trimmed = token.trim_matches(|c: char| matches!(c, ',' | ';' | '|'));
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}

/// Parses a list of notes from the input string.
fn parse_notes_input(value: &str) -> Result<Vec<Note>, String> {
    let mut notes = Vec::new();
    for token in tokens_from_input(value) {
        match Note::parse(&token) {
            Ok(note) => notes.push(note),
            Err(_) => return Err(token),
        }
    }
    Ok(notes)
}
