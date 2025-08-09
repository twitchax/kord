use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

use crate::api::hello;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kord-web.css"/>

        // sets the document title
        <Title text="Kord"/>

        // Router and layout
        <Router>
            <NavBar/>
            <main class="max-w-5xl mx-auto p-4">
                <div class="mt-6">
                    <Routes fallback=|| view! { <p class="text-sm text-red-600">"Page not found."</p> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("about") view=AboutPage/>
                        <Route path=StaticSegment("docs") view=DocsPage/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click_me = move |_| *count.write() += 1;
    let on_click_hello = move |_| {
        let name = "Leptos".to_string();

        spawn_local(async move {
            match hello(name).await {
                Ok(hello_result) => log!("{hello_result}"),
                Err(e) => log!("{e}"),
            }
        });
    };

    view! {
        <h1 class="text-2xl font-semibold tracking-tight">"Welcome to Leptos!"</h1>
        <div class="mt-4 flex gap-3">
            <button class="px-3 py-1.5 rounded bg-indigo-600 text-white hover:bg-indigo-500" on:click=on_click_me>
                "Click Me: " {count}
            </button>
            <button class="px-3 py-1.5 rounded bg-slate-200 hover:bg-slate-300" on:click=on_click_hello>
                "Click Me for a server call"
            </button>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-semibold tracking-tight">"About Kord"</h1>
        <p class="mt-3 text-slate-700">
            "Kord is a music theory library and CLI/web app with ML-powered inference."
        </p>
    }
}

#[component]
fn DocsPage() -> impl IntoView {
    view! {
        <div class="docs-container">
            <header class="text-center mb-12">
                <h1>"Kord Documentation"</h1>
                <p class="docs-subtitle">
                    "A powerful music theory library and CLI tool for Rust and JavaScript with ML-powered inference capabilities."
                </p>
            </header>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"Overview"</h2>
                <p class="text-sage-700 mb-4">
                    "Kord is a comprehensive music theory library that provides both a command-line interface and programmatic APIs for Rust and JavaScript. "
                    "It features machine learning-powered chord recognition, audio analysis, and extensive music theory utilities."
                </p>
                <div class="flex flex-wrap gap-4 mt-6">
                    <span class="px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium">"Chord Analysis"</span>
                    <span class="px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium">"Audio Processing"</span>
                    <span class="px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium">"ML Inference"</span>
                    <span class="px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium">"Cross-Platform"</span>
                    <span class="px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium">"WebAssembly"</span>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"Installation"</h2>
                
                <h3 class="text-xl font-semibold text-sage-700 mb-3">"Binary Installation"</h3>
                
                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"Cargo (Recommended)"</h4>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"$ cargo install kord"</code></pre>
                </div>

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"NPM"</h4>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"$ npm install --save kordweb"</code></pre>
                </div>

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"Wasmer"</h4>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"$ wasmer install twitchax/kord"</code></pre>
                </div>

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"Direct Download"</h4>
                    <p class="text-sage-600 mb-3">"Pre-built binaries are available for:"</p>
                    <ul class="list-disc list-inside text-sage-600 space-y-1">
                        <li>"Windows (x86_64)"</li>
                        <li>"macOS (Apple Silicon & Intel)"</li>
                        <li>"Linux (x86_64)"</li>
                    </ul>
                    <p class="text-sage-600 mt-3">
                        "Download from the "
                        <a href="https://github.com/twitchax/kord/releases/latest" class="text-sage-600 hover:text-sage-700 underline" target="_blank" rel="noreferrer">"latest release"</a>
                        " page."
                    </p>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"CLI Usage"</h2>
                
                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Basic Commands"</h3>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200 mb-4"><code>"$ kord -h

A tool to easily explore music theory principles.

Commands:
  describe  Describes a chord
  play      Describes and plays a chord  
  loop      Loops on a set of chord changes
  guess     Attempt to guess the chord from notes
  analyze   Analyze audio data
  ml        Train and infer with ML"</code></pre>
                </div>

                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Examples"</h3>
                    
                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Describe a Chord"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200 mb-2"><code>"$ kord describe Cmaj7"</code></pre>
                        <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400">
                            <code class="text-sage-700">
                                "Cmaj7" <br/>
                                "   major 7, ionian, first mode of major scale" <br/>
                                "   C, D, E, F, G, A, B" <br/>
                                "   C, E, G, B"
                            </code>
                        </div>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Guess Chord from Notes"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200 mb-2"><code>"$ kord guess C F# D# A"</code></pre>
                        <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400">
                            <code class="text-sage-700">
                                "Cdim" <br/>
                                "   fully diminished, diminished seventh" <br/>
                                "   C, D, E♭, F, G♭, A♭, B𝄫, B" <br/>
                                "   C, E♭, G♭, B𝄫"
                            </code>
                        </div>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Audio Analysis"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200 mb-2"><code>"$ kord analyze mic"</code></pre>
                        <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400">
                            <code class="text-sage-700">
                                "Notes: C3 E3 G3" <br/>
                                "C@3" <br/>
                                "   major" <br/>
                                "   C, D, E, F, G, A, B" <br/>
                                "   C, E, G"
                            </code>
                        </div>
                    </div>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"Library Usage (Rust)"</h2>
                
                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Add to Cargo.toml"</h3>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"[dependencies]
kord = \"*\"  # choose a version"</code></pre>
                </div>

                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Basic Examples"</h3>
                    
                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Create and Analyze Chords"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"use klib::known_chord::KnownChord;
use klib::modifier::Degree;
use klib::note::*;
use klib::chord::*;

// Check chord type
assert_eq!(
    Chord::new(C).augmented().seven().known_chord(), 
    KnownChord::AugmentedDominant(Degree::Seven)
);"</code></pre>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Parse Chords from Strings"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"use klib::base::Parsable;
use klib::note::*;
use klib::chord::*;

// Parse and get scale
let chord = Chord::parse(\"Cm7b5\").unwrap();
assert_eq!(
    chord.scale(), 
    vec![C, D, EFlat, F, GFlat, AFlat, BFlat]
);"</code></pre>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Build Chords Fluently"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"use klib::note::*;
use klib::chord::*;

// Fluid chord building
let chord_tones = C.into_chord()
    .augmented()
    .major7()
    .chord();
    
assert_eq!(chord_tones, vec![C, E, GSharp, B]);"</code></pre>
                    </div>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"JavaScript Usage"</h2>
                
                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Installation & Setup"</h3>
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200 mb-4"><code>"npm install --save kordweb"</code></pre>
                    
                    <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"import init, { KordNote, KordChord } from 'kordweb/klib.js';

// Initialize the WASM module once
await init();"</code></pre>
                </div>

                <div class="mb-6">
                    <h3 class="text-xl font-semibold text-sage-700 mb-3">"Examples"</h3>
                    
                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Working with Notes"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"// Create notes
const note = KordNote.parse('C4');

note.name();    // \"C4\"
note.octave();  // 4"</code></pre>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Building Chords"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"// Parse and build chords
const chord = KordChord.parse('C7#9');

chord.name();        // \"C7(♯9)\"
chord.chordString(); // \"C4 E4 G4 Bb5 D#5\"

// Fluid building
const notes = KordChord.parse('C')
    .minor()
    .seven()
    .chord()
    .map(n => n.name()); 
// [\"C4\", \"Eb4\", \"G4\", \"Bb4\"]"</code></pre>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Chord Transformations"</h4>
                        <pre class="bg-sage-100 p-4 rounded-lg border border-sage-200"><code>"// Transform existing chords
KordChord.parse('C7b9')
    .withOctave(2)
    .chord()
    .map(n => n.name()); 
// [\"C2\", \"D♭2\", \"E2\", \"G2\", \"B♭2\"]"</code></pre>
                    </div>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"Feature Flags"</h2>
                <p class="text-sage-700 mb-4">
                    "Kord supports various feature flags for different use cases and deployment targets:"
                </p>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
                    <div class="bg-sage-50 p-4 rounded-lg border border-sage-200">
                        <h4 class="font-semibold text-sage-800 mb-2">"Core Features"</h4>
                        <ul class="text-sm text-sage-700 space-y-1">
                            <li><code>"cli"</code> - Command-line interface</li>
                            <li><code>"audio"</code> - Audio playback support</li>
                            <li><code>"wasm"</code> - WebAssembly compilation</li>
                            <li><code>"wasi"</code> - WebAssembly System Interface</li>
                        </ul>
                    </div>
                    
                    <div class="bg-sage-50 p-4 rounded-lg border border-sage-200">
                        <h4 class="font-semibold text-sage-800 mb-2">"Analysis & ML"</h4>
                        <ul class="text-sm text-sage-700 space-y-1">
                            <li><code>"analyze"</code> - Audio analysis</li>
                            <li><code>"analyze_mic"</code> - Microphone input</li>
                            <li><code>"analyze_file"</code> - File analysis</li>
                            <li><code>"ml"</code> - Machine learning</li>
                            <li><code>"ml_train"</code> - Model training</li>
                            <li><code>"ml_infer"</code> - Inference</li>
                            <li><code>"ml_gpu"</code> - GPU acceleration</li>
                        </ul>
                    </div>
                </div>
            </div>

            <div class="docs-section">
                <h2 class="text-3xl font-semibold text-sage-800 mb-6">"API Reference"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-4">
                        <a 
                            href="https://docs.rs/kord/latest/klib/"
                            target="_blank"
                            rel="noreferrer"
                            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
                        >
                            <h3 class="text-lg font-semibold text-sage-800 mb-2">"Rust Documentation"</h3>
                            <p class="text-sage-600 text-sm">"Complete API reference for the Rust library"</p>
                        </a>
                        
                        <a 
                            href="https://www.npmjs.com/package/kordweb"
                            target="_blank"
                            rel="noreferrer"
                            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
                        >
                            <h3 class="text-lg font-semibold text-sage-800 mb-2">"NPM Package"</h3>
                            <p class="text-sage-600 text-sm">"JavaScript/TypeScript package information"</p>
                        </a>
                    </div>
                    
                    <div class="space-y-4">
                        <a 
                            href="https://github.com/twitchax/kord"
                            target="_blank"
                            rel="noreferrer"
                            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
                        >
                            <h3 class="text-lg font-semibold text-sage-800 mb-2">"Source Code"</h3>
                            <p class="text-sage-600 text-sm">"View the source code on GitHub"</p>
                        </a>
                        
                        <a 
                            href="https://github.com/twitchax/kord/releases"
                            target="_blank"
                            rel="noreferrer"
                            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
                        >
                            <h3 class="text-lg font-semibold text-sage-800 mb-2">"Releases"</h3>
                            <p class="text-sage-600 text-sm">"Download pre-built binaries"</p>
                        </a>
                    </div>
                </div>
            </div>

            <footer class="text-center mt-12 pt-8 border-t border-sage-200">
                <p class="text-sage-600">
                    "Built with ♪ by the Kord team. Licensed under MIT."
                </p>
            </footer>
        </div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="w-full bg-sage-800 text-white">
            <div class="max-w-5xl mx-auto flex items-center gap-4 px-4 py-3">
                <strong class="mr-2 text-sage-100 font-semibold">"♪ Kord"</strong>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/">"Home"</A>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/about">"About"</A>
                <A attr:class="text-sm text-sage-300 hover:text-sage-100 transition-colors duration-200" href="/docs">"Docs"</A>
            </div>
        </nav>
    }
}
