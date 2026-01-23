use crate::client::shared::{Badge, Callout, CardLink, CodeBlock, PageTitle, Panel, Section, Subheading, TertiaryHeading};
use leptos::prelude::*;
use thaw::{Flex, FlexGap, Text, TextTag};

#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="kord-docs">
            <header class="kord-docs__header">
                <PageTitle attr:style="text-align: center;">"Kord Documentation"</PageTitle>
                <Text tag=TextTag::P>"A powerful music theory library and CLI tool for Rust and JavaScript with ML-powered inference capabilities."</Text>
            </header>

            <Section title="Overview">
                <Text tag=TextTag::P>
                    Kord is a comprehensive music theory library that provides both a command-line interface and programmatic APIs for Rust and JavaScript.
                    It features machine learning-powered chord recognition, audio analysis, and extensive music theory utilities.
                </Text>
                <Flex class="kord-docs__badges">
                    <Badge>"Chord Analysis"</Badge>
                    <Badge>"Audio Processing"</Badge>
                    <Badge>"ML Inference"</Badge>
                    <Badge>"Cross-Platform"</Badge>
                    <Badge>"WebAssembly"</Badge>
                </Flex>
            </Section>

            <Section title="Installation">

                <Subheading text="Binary Installation" />

                <div class="kord-docs__install-section">
                    <TertiaryHeading text="Cargo (Recommended)" />
                    <CodeBlock class="lang-bash" code="$ cargo install kord" />
                </div>

                <div class="kord-docs__install-section">
                    <TertiaryHeading text="NPM" />
                    <CodeBlock class="lang-bash" code="$ npm install --save kordweb" />
                </div>

                <div class="kord-docs__install-section">
                    <TertiaryHeading text="WebAssembly (OCI)" />
                    <CodeBlock class="lang-bash" code="$ wasmtime run ghcr.io/twitchax/kord:latest describe Am7" />
                    <p class="kord-docs__install-text">"Or with wkg:"</p>
                    <CodeBlock class="lang-bash" code="$ wkg get github:twitchax/kord" />
                </div>

                <div class="kord-docs__install-section">
                    <TertiaryHeading text="Direct Download" />
                    <p class="kord-docs__install-text">"Pre-built binaries are available for:"</p>
                    <ul class="kord-docs__platform-list">
                        <li>"Windows (x86_64)"</li>
                        <li>"macOS (Apple Silicon & Intel)"</li>
                        <li>"Linux (x86_64)"</li>
                    </ul>
                    <p class="kord-docs__install-text">
                        "Download from the " <a href="https://github.com/twitchax/kord/releases/latest" class="kord-docs__link" target="_blank" rel="noreferrer">
                            "latest release"
                        </a> " page."
                    </p>
                </div>
            </Section>

            <Section title="CLI Usage">

                <div class="kord-docs__install-section">
                    <Subheading text="Basic Commands" />
                    <CodeBlock
                        class="kord-docs__code-example"
                        code="$ kord -h
                        
                        A tool to easily explore music theory principles.
                        
                        Commands:
                        describe  Describes a chord
                        play      Describes and plays a chord  
                        loop      Loops on a set of chord changes
                        guess     Attempt to guess the chord from notes
                        analyze   Analyze audio data
                        ml        Train and infer with ML"
                    />
                </div>

                <div class="kord-docs__spacer-lg">
                    <Subheading text="Examples" />

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Describe a Chord" />
                        <CodeBlock class="kord-docs__code-example" code="$ kord describe Cmaj7" />
                        <Callout>"Cmaj7" <br /> "   major 7, ionian, first mode of major scale" <br /> "   C, D, E, F, G, A, B" <br /> "   C, E, G, B"</Callout>
                    </div>

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Guess Chord from Notes" />
                        <CodeBlock class="kord-docs__code-example" code="$ kord guess C F# D# A" />
                        <Callout>"Cdim" <br /> "   fully diminished, diminished seventh" <br /> "   C, D, E♭, F, G♭, A♭, B𝄫, B" <br /> "   C, E♭, G♭, B𝄫"</Callout>
                    </div>

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Audio Analysis" />
                        <CodeBlock class="kord-docs__code-example" code="$ kord analyze mic" />
                        <Callout>"Notes: C3 E3 G3" <br /> "C@3" <br /> "   major" <br /> "   C, D, E, F, G, A, B" <br /> "   C, E, G"</Callout>
                    </div>
                </div>
            </Section>

            <Section title="Library Usage (Rust)">

                <div class="kord-docs__spacer-lg">
                    <Subheading text="Add to Cargo.toml" />
                    <CodeBlock code="[dependencies]
                    kord = \"*\"  # choose a version" />
                </div>

                <div class="kord-docs__spacer-lg">
                    <Subheading text="Basic Examples" />

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Create and Analyze Chords" />
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

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Parse Chords from Strings" />
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

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Build Chords Fluently" />
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

                <div class="kord-docs__spacer-lg">
                    <Subheading text="Installation & Setup" />
                    <CodeBlock class="kord-docs__code-example" code="npm install --save kordweb" />

                    <CodeBlock code="import init, { KordNote, KordChord } from 'kordweb/klib.js';
                    
                    // Initialize the WASM module once
                    await init();" />
                </div>

                <div class="kord-docs__spacer-lg">
                    <Subheading text="Examples" />

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Working with Notes" />
                        <CodeBlock code="// Create notes
                        const note = KordNote.parse('C4');
                        
                        note.name();    // \"C4\"
                        note.octave();  // 4" />
                    </div>

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Building Chords" />
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

                    <div class="kord-docs__spacer">
                        <TertiaryHeading text="Chord Transformations" />
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
                <p class="kord-docs__spacer">"Kord supports various feature flags for different use cases and deployment targets:"</p>

                <div class="kord-docs__grid kord-docs__grid--two kord-docs__grid--gap-4">
                    <Panel title="Core Features">
                        <ul>
                            <li>
                                <code>"cli"</code>
                                - Command-line interface
                            </li>
                            <li>
                                <code>"audio"</code>
                                - Audio playback support
                            </li>
                            <li>
                                <code>"wasm"</code>
                                - WebAssembly compilation
                            </li>
                            <li>
                                <code>"wasi"</code>
                                - WebAssembly System Interface
                            </li>
                        </ul>
                    </Panel>

                    <Panel title="Analysis & ML">
                        <ul>
                            <li>
                                <code>"analyze"</code>
                                - Audio analysis
                            </li>
                            <li>
                                <code>"analyze_mic"</code>
                                - Microphone input
                            </li>
                            <li>
                                <code>"analyze_file"</code>
                                - File analysis
                            </li>
                            <li>
                                <code>"ml"</code>
                                - Machine learning
                            </li>
                            <li>
                                <code>"ml_train"</code>
                                - Model training
                            </li>
                            <li>
                                <code>"ml_infer"</code>
                                - Inference
                            </li>
                            <li>
                                <code>"ml_gpu"</code>
                                - GPU acceleration
                            </li>
                        </ul>
                    </Panel>
                </div>
            </Section>

            <Section title="API Reference">
                <div class="kord-docs__grid kord-docs__grid--two kord-docs__grid--gap-6">
                    <Flex vertical=true gap=FlexGap::Medium>
                        <CardLink href="https://docs.rs/kord/latest/klib/" title="Rust Documentation" desc="Complete API reference for the Rust library" />
                        <CardLink href="https://www.npmjs.com/package/kordweb" title="NPM Package" desc="JavaScript/TypeScript package information" />
                    </Flex>

                    <Flex vertical=true gap=FlexGap::Medium>
                        <CardLink href="https://github.com/twitchax/kord" title="Source Code" desc="View the source code on GitHub" />
                        <CardLink href="https://github.com/twitchax/kord/releases" title="Releases" desc="Download pre-built binaries" />
                    </Flex>
                </div>
            </Section>

            <footer>
                <p>
                    "Built with ♪ by the Kord team. Licensed under " <a href="https://github.com/twitchax/kord/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">
                        "MIT"
                    </a> "."
                </p>
            </footer>
        </div>
    }
}
