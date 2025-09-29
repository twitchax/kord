[![Build and Test](https://github.com/twitchax/kord/actions/workflows/build.yml/badge.svg)](https://github.com/twitchax/kord/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/twitchax/kord/branch/main/graph/badge.svg?token=35MZN0YFZF)](https://codecov.io/gh/twitchax/kord)
[![Version](https://img.shields.io/crates/v/kord.svg)](https://crates.io/crates/kord)
[![Crates.io](https://img.shields.io/crates/d/kord?label=crate)](https://crates.io/crates/kord)
[![GitHub all releases](https://img.shields.io/github/downloads/twitchax/kord/total?label=binary)](https://github.com/twitchax/kord/releases)
[![npm](https://img.shields.io/npm/dt/kordweb?label=npm)](https://www.npmjs.com/package/kordweb)
[![Documentation](https://docs.rs/kord/badge.svg)](https://docs.rs/kord)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# kord

A music theory binary and library for Rust / JS (via WASM) ([capability playground](https://kord.twitchax.com/)).

> **Note**: The [`kord-web`](kord-web/) directory contains a web application that serves as a demonstration and example of how to integrate the Kord library with modern web frameworks. It is not part of the core library but showcases the library's WASM capabilities in a real-world web application (and it is the example website linked above).

## Binary Usage

### Install

Windows:

```powershell
$ iwr https://github.com/twitchax/kord/releases/latest/download/kord_x86_64-pc-windows-gnu.zip
$ Expand-Archive kord_x86_64-pc-windows-gnu.zip -DestinationPath C:\Users\%USERNAME%\AppData\Local\Programs\kord
```

Mac OS (Apple Silicon):

```bash
$ curl -LO https://github.com/twitchax/kord/releases/latest/download/kord_aarch64-apple-darwin.zip
$ unzip kord_aarch64-apple-darwin.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/kord
```

Linux:

```bash
$ curl -LO https://github.com/twitchax/kord/releases/latest/download/kord_x86_64-unknown-linux-gnu.zip
$ unzip kord_x86_64-unknown-linux-gnu.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/kord
```

Cargo:

```bash
$ cargo install kord
```

NPM:

```bash
$ npm install --save kordweb
```

### Wasmer

This has a reduced capability set (no audio input / output), but works well for some of the core use cases.

```bash
$ wasmer install twitchax/kord
```

Alternatively, you can use `wasmer run`.

```bash
$ wasmer run twitchax/kord -- describe Am7
```

### Help Docs

```bash
$ kord -h

A tool to easily explore music theory principles.

Usage: kord.exe [COMMAND]

Commands:
  describe  Describes a chord
  play      Describes and plays a chord
  loop      Loops on a set of chord changes, while simultaneously outputting the descriptions
  guess     Attempt to guess the chord from a set of notes (ordered by simplicity)
  analyze   Set of commands to analyze audio data
  ml        Set of commands to train and infer with ML
  help      Print this message or the help of the given subcommand(s)

Options:
   -v, --verbose  Flag that specifies verbose logging
   -q, --quiet    Flag that suppresses all tracing output
   -h, --help     Print help information
   -V, --version  Print version information
```

### Describe A Chord

```bash
$ kord describe Cmaj7

Cmaj7
   major 7, ionian, first mode of major scale
   C, D, E, F, G, A, B
   C, E, G, B
```

### Play A Chord

```bash
$ kord play Bb7#9#11

B♭7(♯9)(♯11)
   dominant sharp 9, altered, altered dominant, super locrian, diminished whole tone, seventh mode of a melodic minor scale, melodic minor up a half step
   B♭, C♭, D♭, E𝄫, F♭, G♭, A♭
   B♭, D, F, A♭, C♯, E
```

### Loop Through Chord Changes

```bash
$ kord loop -b 120 "Em7b5@3^2" "A7b13@3!" "D-maj7@3^2" "G7@3" "Cmaj7@3^2"
```

### Guess A Chord

```bash
$ kord guess C F# D# A
Cdim
   fully diminished (whole first), diminished seventh, whole/half/whole diminished
   C, D, E♭, F, G♭, A♭, B𝄫, B
   C, E♭, G♭, B𝄫
Cm(♭5)(add6)
   minor
   C, D, E♭, F, G, A♭, B♭
   C, E♭, G♭, A
```

```bash
$ kord guess C G Bb F#5 F
C7(♯11)(sus4)
   dominant sharp 11, lydian dominant, lyxian, major with sharp four and flat seven
   C, D, E, F♯, G, A, B♭
   C, F, G, B♭, F♯
Cm7(♯11)(sus4)
   minor 7, dorian, second mode of major scale, major with flat third and flat seven
   C, D, E♭, F, G, A, B♭
   C, F, G, B♭, F♯
```

```bash
$ kord guess E3 C4 Eb4 F#4 A#4 D5 D4
Cm9(♭5)(add2)/E
   half diminished, locrian, minor seven flat five, seventh mode of major scale, major scale starting one half step up
   C, D, E♭, F, G♭, A♭, B♭
   E, C, D, E♭, G♭, B♭, D
```

### Guess Notes / Chord From Audio

Using the deterministic algorithm only:

```bash
$ kord analyze mic

Notes: C3 E3 G3
C@3
   major
   C, D, E, F, G, A, B
   C, E, G
```

Using the ML algorithm:

```bash
$ kord ml infer mic

Notes: C3 E3 G3
C@3
   major
   C, D, E, F, G, A, B
   C, E, G
```

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
kord = "*" #choose a version
```

### Examples

```rust
use klib::core::known_chord::KnownChord;
use klib::core::modifier::Degree;
use klib::core::note::*;
use klib::core::chord::*;

// Check to see what _kind_ of chord this is.
assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));
```

```rust
use klib::core::base::Parsable;
use klib::core::note::*;
use klib::core::chord::*;

// Parse a chord from a string, and inspect the scale.
assert_eq!(Chord::parse("Cm7b5").unwrap().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);
```

```rust
use klib::core::note::*;
use klib::core::chord::*;

// From a note, create a chord, and look at the chord tones.
assert_eq!(C.into_chord().augmented().major7().chord(), vec![C, E, GSharp, B]);
```

## JS Usage

The npm package is available [here](https://www.npmjs.com/package/kordweb).

First, load the module as you would any other ES module.

```js
import init, { KordNote, KordChord } from 'kordweb/klib.js';

// Run `init` once.
await init();
```

Then, you can use the library similarly as you would in Rust.

```js
// Create a note.
const note = KordNote.parse('C4');

note.name(); // C4
note.octave(); // 4

// Create a chord.
const chord = KordChord.parse('C7#9');

chord.name(); // C7(♯9)
chord.chordString(); // C4 E4 G4 Bb5 D#5

// Easy chaining.
KordChord.parse('C7b9').withOctave(2).chord().map(n => n.name()); // [ 'C2', 'D♭2', 'E2', 'G2', 'B♭2' ]

// Build chords.
KordChord.parse('C').minor().seven().chord().map(n => n.name()); // [ 'C4', 'Eb4', 'G4', 'Bb4' ]
```

## Feature Flags

The library and binary both support various feature flags.  Of most important note are:
* `default = ["cli", "analyze", "audio"]`
* `cli`: enables the CLI features, and can be removed if only compiling the library.
* `analyze = ["analyze_mic", "analyze_file"]`: enables the `analyze` subcommand, which allows for analyzing audio data (and the underlying library features).
  * `analyze_mic`: enables the `analyze mic` subcommand, which allows for analyzing audio from a microphone (and the underlying library features).
  * `analyze_file`: enables the `analyze file` subcommand, which allows for analyzing audio from a file (and the underlying library features).
    * `analyze_file_mp3`: enables the features to analyze mp3 files.
    * `analyze_file_aac`: enables the features to analyze aac files.
    * `analyze_file_alac`: enables the features to analyze alac files.
* `ml = ["ml_train", "ml_infer"]`: enables the `ml` subcommand, which allows for training and inferring with ML (and the underlying library features).
  * `ml_train`: enables the `ml train` subcommand, which allows for training ML models (and the underlying library features).
  * `ml_infer`: enables the `ml infer` subcommand, which allows for inferring with ML models (and the underlying library features).
    * > NOTE: Adding the `analyze_mic` feature flag will enable the `ml infer mic` subcommand, which allows for inferring with ML models from a microphone.
    * > NOTE: Adding the `analyze_file` feature flag will enable the `ml infer file` subcommand, which allows for inferring with ML models from a file.
  * `ml_gpu`: enables the features to use a GPU for ML _training_.
  * `ml_sample_gather`: enables the `ml gather` subcommand for capturing raw samples from a microphone (requires `analyze_mic`).
  * `ml_hpt`: enables the `ml hpt` subcommand for sweeping hyperparameters (implies `ml_train`).
* `ml_sample_process`: enables the `ml process` subcommand for generating samples from aligned MIDI and audio files, naming each emitted sample with its originating measure index, rounded duration (in seconds), and chord tones.
* `wasm`: enables the features to compile to wasm.
* `plot`: enables the features to plot data.

### ML Loader & Target Feature Matrix

The ML pipeline exposes toggles that change both the training inputs and labels without modifying source code. These features can be mixed and matched to explore alternative data representations:

#### Sample Loader Features (choose exactly one)

| Feature                             | Description                                                    | Input width (before deterministic guess) |
| ----------------------------------- | -------------------------------------------------------------- | ---------------------------------------- |
| `ml_loader_note_binned_convolution` | Uses the existing note-binned harmonic convolution (128 bins). | 128                                      |
| `ml_loader_mel`                     | Applies mel filter banks to the full spectrum (512 bands).     | 512                                      |
| `ml_loader_frequency`               | Feeds the raw 8,192-bin frequency spectrum.                    | 8192                                     |

Optional add-on:

| Feature                                 | Description                                                                                                                                  |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `ml_loader_include_deterministic_guess` | Prepends the deterministic 128-note guess vector to whichever loader you selected above (doubling 128-bin inputs, adding 128 to the others). |

#### Target Encoding Features (enable one or both)

| Feature            | Description                                                  | Output width contribution |
| ------------------ | ------------------------------------------------------------ | ------------------------- |
| `ml_target_full`   | Emits the full 128-note mask (per MIDI note across octaves). | +128                      |
| `ml_target_folded` | Emits a folded 12-class pitch-class mask (one octave).       | +12                       |

When both target features are enabled, the model receives the 128-note mask followed by the 12-class folded mask in a single output vector (`TARGET_SPACE_SIZE = 140`).

#### Example configurations

```bash
# Default (note-binned + deterministic guess, 128-note target)
cargo check

# Mel features with deterministic guess and combined targets
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_mel ml_loader_include_deterministic_guess ml_target_full ml_target_folded"

# Raw frequency spectrum without deterministic guess, folded targets only
cargo check --no-default-features \
   --features "cli ml_infer ml_loader_frequency ml_target_folded"
```

> Make sure exactly one loader feature is enabled at a time. The deterministic guess flag and target features can be toggled independently to suit experiments.

## Test

```bash
cargo test
```

## License

MIT