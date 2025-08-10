use leptos::prelude::*;

#[component]
fn Badge(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base = "px-3 py-1 bg-sage-100 text-sage-800 rounded-full text-sm font-medium select-none";
    let cls = class
        .map(|c| format!("{base} {c}"))
        .unwrap_or_else(|| base.to_string());
    view! { <span class=cls>{children()}</span> }
}

#[component]
fn Section(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="docs-section">
            <h2 class="text-3xl font-semibold text-sage-800 mb-6">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
fn Subheading(#[prop(into)] text: String) -> impl IntoView {
    view! { <h3 class="text-xl font-semibold text-sage-700 mb-3">{text}</h3> }
}

#[component]
fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let base = "bg-sage-100 p-4 rounded-lg border border-sage-200";
    let cls = class
        .map(|c| format!("{base} {c}"))
        .unwrap_or_else(|| base.to_string());
    view! { <pre class=cls><code>{code}</code></pre> }
}

#[component]
fn CardLink(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noreferrer"
            class="block p-4 bg-white border border-sage-200 rounded-lg hover:border-sage-300 transition-all duration-200 hover:shadow-md"
        >
            <h3 class="text-lg font-semibold text-sage-800 mb-2">{title}</h3>
            <p class="text-sage-600 text-sm">{desc}</p>
        </a>
    }
}

#[component]
fn Callout(children: Children) -> impl IntoView {
    view! { <div class="bg-sage-50 p-3 rounded border-l-4 border-sage-400"><code class="text-sage-700">{children()}</code></div> }
}

#[component]
fn Panel(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-sage-50 p-4 rounded-lg border border-sage-200">
            <h4 class="font-semibold text-sage-800 mb-2">{title}</h4>
            {children()}
        </div>
    }
}

#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="docs-container">
            <header class="text-center mb-12">
                <h1>"Kord Documentation"</h1>
                <p class="docs-subtitle">
                    "A powerful music theory library and CLI tool for Rust and JavaScript with ML-powered inference capabilities."
                </p>
            </header>

            <Section title="Overview">
                <p class="text-sage-700 mb-4">
                    "Kord is a comprehensive music theory library that provides both a command-line interface and programmatic APIs for Rust and JavaScript. "
                    "It features machine learning-powered chord recognition, audio analysis, and extensive music theory utilities."
                </p>
                <div class="flex flex-wrap gap-4 mt-6">
                    <Badge>"Chord Analysis"</Badge>
                    <Badge>"Audio Processing"</Badge>
                    <Badge>"ML Inference"</Badge>
                    <Badge>"Cross-Platform"</Badge>
                    <Badge>"WebAssembly"</Badge>
                </div>
            </Section>

            <Section title="Installation">

                <Subheading text="Binary Installation" />

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"Cargo (Recommended)"</h4>
                    <CodeBlock code="$ cargo install kord" />
                </div>

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"NPM"</h4>
                    <CodeBlock code="$ npm install --save kordweb" />
                </div>

                <div class="mb-6">
                    <h4 class="text-lg font-medium text-sage-700 mb-2">"Wasmer"</h4>
                    <CodeBlock code="$ wasmer install twitchax/kord" />
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
            </Section>

            <Section title="CLI Usage">

                <div class="mb-6">
                    <Subheading text="Basic Commands" />
                    <CodeBlock class="mb-4" code="$ kord -h

A tool to easily explore music theory principles.

Commands:
  describe  Describes a chord
  play      Describes and plays a chord  
  loop      Loops on a set of chord changes
  guess     Attempt to guess the chord from notes
  analyze   Analyze audio data
  ml        Train and infer with ML" />
                </div>

                <div class="mb-6">
                    <Subheading text="Examples" />

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Describe a Chord"</h4>
                        <CodeBlock class="mb-2" code="$ kord describe Cmaj7" />
                        <Callout>
                            "Cmaj7" <br/>
                            "   major 7, ionian, first mode of major scale" <br/>
                            "   C, D, E, F, G, A, B" <br/>
                            "   C, E, G, B"
                        </Callout>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Guess Chord from Notes"</h4>
                        <CodeBlock class="mb-2" code="$ kord guess C F# D# A" />
                        <Callout>
                            "Cdim" <br/>
                            "   fully diminished, diminished seventh" <br/>
                            "   C, D, E♭, F, G♭, A♭, B𝄫, B" <br/>
                            "   C, E♭, G♭, B𝄫"
                        </Callout>
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Audio Analysis"</h4>
                        <CodeBlock class="mb-2" code="$ kord analyze mic" />
                        <Callout>
                            "Notes: C3 E3 G3" <br/>
                            "C@3" <br/>
                            "   major" <br/>
                            "   C, D, E, F, G, A, B" <br/>
                            "   C, E, G"
                        </Callout>
                    </div>
                </div>
            </Section>

            <Section title="Library Usage (Rust)">

                <div class="mb-6">
                    <Subheading text="Add to Cargo.toml" />
                    <CodeBlock code="[dependencies]
    kord = \"*\"  # choose a version" />
                </div>

                <div class="mb-6">
                    <Subheading text="Basic Examples" />

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Create and Analyze Chords"</h4>
                        <CodeBlock code="use klib::known_chord::KnownChord;
use klib::modifier::Degree;
use klib::note::*;
use klib::chord::*;

// Check chord type
assert_eq!(
    Chord::new(C).augmented().seven().known_chord(), 
    KnownChord::AugmentedDominant(Degree::Seven)
);" />
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Parse Chords from Strings"</h4>
                        <CodeBlock code="use klib::base::Parsable;
use klib::note::*;
use klib::chord::*;

// Parse and get scale
let chord = Chord::parse(\"Cm7b5\").unwrap();
assert_eq!(
    chord.scale(), 
    vec![C, D, EFlat, F, GFlat, AFlat, BFlat]
);" />
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Build Chords Fluently"</h4>
                        <CodeBlock code="use klib::note::*;
use klib::chord::*;

// Fluid chord building
let chord_tones = C.into_chord()
    .augmented()
    .major7()
    .chord();
    
assert_eq!(chord_tones, vec![C, E, GSharp, B]);" />
                    </div>
                </div>
            </Section>

            <Section title="JavaScript Usage">

                <div class="mb-6">
                    <Subheading text="Installation & Setup" />
                    <CodeBlock class="mb-4" code="npm install --save kordweb" />

                    <CodeBlock code="import init, { KordNote, KordChord } from 'kordweb/klib.js';

// Initialize the WASM module once
await init();" />
                </div>

                <div class="mb-6">
                    <Subheading text="Examples" />

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Working with Notes"</h4>
                        <CodeBlock code="// Create notes
const note = KordNote.parse('C4');

note.name();    // \"C4\"
note.octave();  // 4" />
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Building Chords"</h4>
                        <CodeBlock code="// Parse and build chords
const chord = KordChord.parse('C7#9');

chord.name();        // \"C7(♯9)\"
chord.chordString(); // \"C4 E4 G4 Bb5 D#5\"

// Fluid building
const notes = KordChord.parse('C')
    .minor()
    .seven()
    .chord()
    .map(n => n.name()); 
// [\"C4\", \"Eb4\", \"G4\", \"Bb4\"]" />
                    </div>

                    <div class="mb-4">
                        <h4 class="text-lg font-medium text-sage-700 mb-2">"Chord Transformations"</h4>
                        <CodeBlock code="// Transform existing chords
KordChord.parse('C7b9')
    .withOctave(2)
    .chord()
    .map(n => n.name()); 
// [\"C2\", \"D♭2\", \"E2\", \"G2\", \"B♭2\"]" />
                    </div>
                </div>
            </Section>

            <Section title="Feature Flags">
                <p class="text-sage-700 mb-4">
                    "Kord supports various feature flags for different use cases and deployment targets:"
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
                    <Panel title="Core Features">
                        <ul class="text-sm text-sage-700 space-y-1">
                            <li><code>"cli"</code> - Command-line interface</li>
                            <li><code>"audio"</code> - Audio playback support</li>
                            <li><code>"wasm"</code> - WebAssembly compilation</li>
                            <li><code>"wasi"</code> - WebAssembly System Interface</li>
                        </ul>
                    </Panel>

                    <Panel title="Analysis & ML">
                        <ul class="text-sm text-sage-700 space-y-1">
                            <li><code>"analyze"</code> - Audio analysis</li>
                            <li><code>"analyze_mic"</code> - Microphone input</li>
                            <li><code>"analyze_file"</code> - File analysis</li>
                            <li><code>"ml"</code> - Machine learning</li>
                            <li><code>"ml_train"</code> - Model training</li>
                            <li><code>"ml_infer"</code> - Inference</li>
                            <li><code>"ml_gpu"</code> - GPU acceleration</li>
                        </ul>
                    </Panel>
                </div>
            </Section>

            <Section title="API Reference">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-4">
                        <CardLink href="https://docs.rs/kord/latest/klib/" title="Rust Documentation" desc="Complete API reference for the Rust library" />
                        <CardLink href="https://www.npmjs.com/package/kordweb" title="NPM Package" desc="JavaScript/TypeScript package information" />
                    </div>

                    <div class="space-y-4">
                        <CardLink href="https://github.com/twitchax/kord" title="Source Code" desc="View the source code on GitHub" />
                        <CardLink href="https://github.com/twitchax/kord/releases" title="Releases" desc="Download pre-built binaries" />
                    </div>
                </div>
            </Section>

            <footer class="text-center mt-12 pt-8 border-t border-sage-200">
                <p class="text-sage-600">
                    "Built with ♪ by the Kord team. Licensed under "
                    <a href="https://github.com/twitchax/kord/blob/main/LICENSE" target="_blank" rel="noopener noreferrer" class="underline hover:text-sage-800 transition-colors duration-200">"MIT"</a>
                    "."
                </p>
            </footer>
        </div>
    }
}
