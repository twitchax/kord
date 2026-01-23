use crate::client::shared::{Badge, PageTitle, PrimaryButton, SecondaryButton};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let navigate = use_navigate();

    let listen_cta = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/listen", Default::default());
        }
    };

    let listen_feature = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/listen", Default::default());
        }
    };

    let docs_cta = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/docs", Default::default());
        }
    };

    let docs_feature = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/docs", Default::default());
        }
    };

    let describe_meta = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/describe", Default::default());
        }
    };

    let describe_feature = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/describe", Default::default());
        }
    };

    let guess_meta = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/guess", Default::default());
        }
    };

    let guess_feature = {
        let navigate = navigate.clone();
        move |_| {
            navigate("/guess", Default::default());
        }
    };

    view! {
        <section class="kord-home">
            <div class="kord-home__hero">
                <Badge class="kord-home__badge">"Rust · WASM · CLI"</Badge>
                <PageTitle>"Music theory helpers that ship with your code."</PageTitle>
                <p class="kord-home__lead">
                    "Kord is an open-source Rust library, CLI, and WASM module for parsing chord symbols, spelling notes, and inferring chords from pitches or short audio clips."
                    "  This site is a thin demo of the same APIs compiled to the browser."
                </p>
                <div class="kord-home__cta">
                    <PrimaryButton class="kord-home__cta-button" on_click=listen_cta>
                        "Open Listen demo"
                    </PrimaryButton>
                    <SecondaryButton class="kord-home__cta-button" on_click=docs_cta>
                        "Read the docs"
                    </SecondaryButton>
                </div>
                <p class="kord-home__note">"Inspect chord grammar with Describe, test voicings in Guess, or follow the docs to pull the crate and CLI into your own projects."</p>
                <div class="kord-home__meta">
                    <button class="kord-home__meta-link" type="button" on:click=describe_meta>
                        "Describe a chord"
                    </button>
                    <span class="kord-home__meta-sep">"•"</span>
                    <button class="kord-home__meta-link" type="button" on:click=guess_meta>
                        "Guess from notes"
                    </button>
                    <span class="kord-home__meta-sep">"•"</span>
                    <a class="kord-home__meta-link" href="https://github.com/twitchax/kord" target="_blank" rel="noopener noreferrer">
                        "View on GitHub"
                    </a>
                </div>
            </div>

            <div class="kord-home__feature-grid">
                <article class="kord-home__feature">
                    <h3 class="kord-home__feature-title">"Describe complex symbols"</h3>
                    <p class="kord-home__feature-desc">"Instantly expand Cm7(#11)/G or altered dominants into precise spellings, suggested scales, and playback."</p>
                    <SecondaryButton class="kord-home__feature-action" on_click=describe_feature>
                        "Open Describe"
                    </SecondaryButton>
                </article>
                <article class="kord-home__feature">
                    <h3 class="kord-home__feature-title">"Guess chords from notes"</h3>
                    <p class="kord-home__feature-desc">"Enter pitches or tap the built-in keyboard to surface voicings ranked by musical fit."</p>
                    <SecondaryButton class="kord-home__feature-action" on_click=guess_feature>
                        "Try Guess"
                    </SecondaryButton>
                </article>
                <article class="kord-home__feature">
                    <h3 class="kord-home__feature-title">"Listen in real time"</h3>
                    <p class="kord-home__feature-desc">"Record a quick snippet and let our ML model surface the chords behind the sound."</p>
                    <SecondaryButton class="kord-home__feature-action" on_click=listen_feature>
                        "Launch Listen"
                    </SecondaryButton>
                </article>
                <article class="kord-home__feature">
                    <h3 class="kord-home__feature-title">"Learn the internals"</h3>
                    <p class="kord-home__feature-desc">"Dive into the documentation for CLI commands, WASM bindings, and theory notes that power the experience."</p>
                    <SecondaryButton class="kord-home__feature-action" on_click=docs_feature>
                        "Read Docs"
                    </SecondaryButton>
                </article>
            </div>
        </section>
    }
}
