use crate::client::shared::{ChordAnalysis, PageTitle};
use klib::core::{base::Parsable, chord::Chord};
use leptos::prelude::*;
use std::time::Duration;
use thaw::{Field, FieldValidationState, Flex, FlexGap, Input, InputRule};

const DEBOUNCE_MS: u64 = 300;

#[component]
pub fn DescribePage() -> impl IntoView {
    let chord_input = RwSignal::new(String::new());
    let chord_result = RwSignal::new(None);

    let mut debounced_parse = leptos::prelude::debounce(Duration::from_millis(DEBOUNCE_MS), move |val: String| {
        let trimmed = val.trim();
        if trimmed.is_empty() {
            chord_result.set(None);
        } else {
            chord_result.set(Chord::parse(trimmed).ok());
        }
    });

    // Watch input changes and trigger debounced parsing.
    Effect::watch(
        move || chord_input.get(),
        move |val, _, _| {
            debounced_parse(val.clone());
        },
        false,
    );

    // Validation rules for the chord input.
    let required_rule = InputRule::required_with_message(Signal::derive(|| true), "Required".into());
    let chord_parse_rule = InputRule::validator(|v: &String, _| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        if Chord::parse(trimmed).is_ok() {
            Ok(())
        } else {
            Err(FieldValidationState::Error("Unrecognized chord".to_string()))
        }
    });
    let rules = vec![required_rule, chord_parse_rule];

    view! {
    <PageTitle>"Describe a Chord"</PageTitle>
    <Flex vertical=true gap=FlexGap::Large class="kord-content__section">
            <Field label="Chord Symbol">
                <Input
                    id="describe-chord"
                    placeholder="e.g. Cm7"
                    value=chord_input
                    rules=rules
                />
            </Field>
        </Flex>

        {move || chord_result.get().map(|c| view! { <ChordAnalysis chord=c /> })}
    }
}
