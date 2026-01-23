use crate::client::shared::{NotationAnalysis, PageTitle};
use klib::core::{base::Parsable, notation::Notation};
use leptos::prelude::*;
use std::time::Duration;
use thaw::{Field, FieldValidationState, Flex, FlexGap, Input, InputRule};

const DEBOUNCE_MS: u64 = 300;

/// Parse a notation (chord, scale, or mode) from user input, handling empty/whitespace strings.
pub fn parse_notation_input(input: &str) -> Option<Notation> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Notation::parse(trimmed).ok()
    }
}

#[component]
pub fn DescribePage() -> impl IntoView {
    let notation_input = RwSignal::new(String::new());
    let notation_result = RwSignal::new(None);

    let mut debounced_parse = leptos::prelude::debounce(Duration::from_millis(DEBOUNCE_MS), move |val: String| {
        notation_result.set(parse_notation_input(&val));
    });

    // Watch input changes and trigger debounced parsing.
    Effect::watch(
        move || notation_input.get(),
        move |val, _, _| {
            debounced_parse(val.clone());
        },
        false,
    );

    // Validation rules for the notation input.
    let required_rule = InputRule::required_with_message(Signal::derive(|| true), "Required".into());
    let notation_parse_rule = InputRule::validator(|v: &String, _| {
        let trimmed = v.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        if Notation::parse(trimmed).is_ok() {
            Ok(())
        } else {
            Err(FieldValidationState::Error("Unrecognized notation".to_string()))
        }
    });
    let rules = vec![required_rule, notation_parse_rule];

    // Derive the kind indicator text.
    let kind_indicator = Signal::derive(move || {
        notation_result.with(|r| {
            r.as_ref().map(|n| match n {
                Notation::Chord(_) => "Chord",
                Notation::Scale(_) => "Scale",
                Notation::Mode(_) => "Mode",
            })
        })
    });

    view! {
        <PageTitle>"Describe"</PageTitle>
        <section class="kord-describe">
            <Flex vertical=true gap=FlexGap::Large>
                <Flex
                    vertical=true
                    gap=FlexGap::Medium
                    class="kord-content__section kord-describe__card"
                >
                    <div class="kord-describe__hint">
                        <p>"Type any chord, scale, or mode to see its full breakdown."</p>
                        <p>"Examples: Cm7, D dorian, A harmonic minor, C major pentatonic"</p>
                    </div>
                    <div class="kord-describe__field">
                        <Field label="Chord / Scale / Mode">
                            <Input
                                id="describe-notation"
                                placeholder="e.g. Cm7, D dorian, A harmonic minor"
                                value=notation_input
                                rules=rules
                            />
                        </Field>
                    </div>
                </Flex>

                <div class="kord-content__section kord-describe__results">
                    <Flex justify=thaw::FlexJustify::SpaceBetween align=thaw::FlexAlign::Center>
                        <h3 class="kord-describe__results-title">"Breakdown"</h3>
                        <Show when=move || kind_indicator.get().is_some()>
                            <span class="kord-describe__kind-badge">
                                {move || kind_indicator.get().unwrap_or("")}
                            </span>
                        </Show>
                    </Flex>
                    <Show
                        when=move || notation_result.with(|result| result.is_some())
                        fallback=move || {
                            view! {
                                <p class="kord-describe__empty">
                                    "Enter a chord, scale, or mode to preview its structure."
                                </p>
                            }
                                .into_view()
                        }
                    >
                        {move || {
                            let notation = notation_result
                                .get()
                                .expect("notation exists when show renders");
                            view! { <NotationAnalysis notation=notation /> }
                        }}
                    </Show>
                </div>
            </Flex>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_notation_input_chord() {
        let result = parse_notation_input("C");
        assert!(result.is_some());
        assert!(result.unwrap().is_chord());

        let result = parse_notation_input("Cm7");
        assert!(result.is_some());
        assert!(result.unwrap().is_chord());

        let result = parse_notation_input("C7#9b5");
        assert!(result.is_some());
        assert!(result.unwrap().is_chord());
    }

    #[test]
    fn test_parse_notation_input_scale() {
        let result = parse_notation_input("C major pentatonic");
        assert!(result.is_some());
        assert!(result.unwrap().is_scale());

        let result = parse_notation_input("A harmonic minor");
        assert!(result.is_some());
        assert!(result.unwrap().is_scale());
    }

    #[test]
    fn test_parse_notation_input_mode() {
        let result = parse_notation_input("D dorian");
        assert!(result.is_some());
        assert!(result.unwrap().is_mode());

        let result = parse_notation_input("F lydian");
        assert!(result.is_some());
        assert!(result.unwrap().is_mode());
    }

    #[test]
    fn test_parse_notation_input_with_whitespace() {
        assert!(parse_notation_input(" Cm7 ").is_some());
        assert!(parse_notation_input("\tD dorian\n").is_some());
    }

    #[test]
    fn test_parse_notation_input_empty() {
        assert!(parse_notation_input("").is_none());
        assert!(parse_notation_input("   ").is_none());
        assert!(parse_notation_input("\t\n").is_none());
    }

    #[test]
    fn test_parse_notation_input_invalid() {
        assert!(parse_notation_input("InvalidNotation123").is_none());
        assert!(parse_notation_input("XYZ").is_none());
    }
}
