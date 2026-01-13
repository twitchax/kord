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
    let note_count = Signal::derive(move || tokens_from_input(&notes_input.get()).len());

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
        <section class="kord-guess">
            <Flex vertical=true gap=FlexGap::Large>
                <Flex vertical=true gap=FlexGap::Medium class="kord-content__section kord-guess__form">
                    <div class="kord-guess__hint">
                        <p>"Describe a voicing with text or click the keyboard—notes are kept unique automatically."</p>
                        <p>"Separate notes with spaces, commas, or tap them in any order."</p>
                    </div>
                    <div class="kord-guess__field">
                        <Field label="Notes">
                            <Input
                                id="guess-notes"
                                placeholder="e.g. C E G Bb"
                                value=notes_input
                                rules=rules
                            />
                        </Field>
                    </div>
                </Flex>

                <div class="kord-content__section kord-guess__stage">
                    <h3 class="kord-guess__stage-title">"Play the Notes"</h3>
                    <p class="kord-guess__stage-subtitle">"Use the piano to lock in the pitches you hear."</p>
                    <Piano on_key_press=on_piano_key_press />
                </div>

                <div class="kord-content__section kord-guess__results">
                    <h3 class="kord-guess__results-title">"Candidate Chords"</h3>
                    <Show
                        when=move || !chords.with(|candidates| candidates.is_empty())
                        fallback=move || {
                            let message = match note_count.get() {
                                0 => "Enter some notes or tap the keyboard to begin.",
                                1 => "Add at least one more note to unlock chord suggestions.",
                                _ => "No matches yet—try tweaking the voicing or adding more notes.",
                            };
                            view! { <p class="kord-guess__empty">{message}</p> }.into_view()
                        }
                    >
                        {move || {
                            let rendered = chords
                                .get()
                                .into_iter()
                                .map(|c| view! { <ChordAnalysis chord=c /> })
                                .collect_view();
                            view! { <div class="kord-guess__chords">{rendered}</div> }.into_view()
                        }}
                    </Show>
                </div>
            </Flex>
        </section>
    }
}

/// Splits the input string into tokens based on whitespace and common delimiters.
pub fn tokens_from_input(value: &str) -> Vec<String> {
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
pub fn parse_notes_input(value: &str) -> Result<Vec<Note>, String> {
    let mut notes = Vec::new();
    for token in tokens_from_input(value) {
        match Note::parse(&token) {
            Ok(note) => notes.push(note),
            Err(_) => return Err(token),
        }
    }
    Ok(notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_from_input_whitespace() {
        assert_eq!(tokens_from_input("C E G"), vec!["C", "E", "G"]);
        assert_eq!(tokens_from_input("C  E   G"), vec!["C", "E", "G"]);
        assert_eq!(tokens_from_input("C\tE\nG"), vec!["C", "E", "G"]);
    }

    #[test]
    fn test_tokens_from_input_delimiters() {
        assert_eq!(tokens_from_input("C,E,G"), vec!["C", "E", "G"]);
        assert_eq!(tokens_from_input("C;E;G"), vec!["C", "E", "G"]);
        assert_eq!(tokens_from_input("C|E|G"), vec!["C", "E", "G"]);
        assert_eq!(tokens_from_input("C, E, G"), vec!["C", "E", "G"]);
    }

    #[test]
    fn test_tokens_from_input_mixed() {
        assert_eq!(tokens_from_input("C E, G; Bb | D"), vec!["C", "E", "G", "Bb", "D"]);
    }

    #[test]
    fn test_tokens_from_input_empty() {
        assert_eq!(tokens_from_input(""), Vec::<String>::new());
        assert_eq!(tokens_from_input("   "), Vec::<String>::new());
        assert_eq!(tokens_from_input(",,,"), Vec::<String>::new());
    }

    #[test]
    fn test_parse_notes_input_valid() {
        let result = parse_notes_input("C E G");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_parse_notes_input_with_accidentals() {
        let result = parse_notes_input("C# Eb F# Bb");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn test_parse_notes_input_invalid() {
        let result = parse_notes_input("C X G");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "X");
    }

    #[test]
    fn test_parse_notes_input_empty() {
        let result = parse_notes_input("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_notes_input_delimiters() {
        let result = parse_notes_input("C, E, G, Bb");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }
}
