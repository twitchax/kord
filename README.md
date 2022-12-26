[![Build and Test](https://github.com/twitchax/kord/actions/workflows/build.yml/badge.svg)](https://github.com/twitchax/kord/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/twitchax/kord/branch/main/graph/badge.svg?token=35MZN0YFZF)](https://codecov.io/gh/twitchax/kord)
[![Version](https://img.shields.io/crates/v/kord.svg)](https://crates.io/crates/kord)
[![Downloads](https://img.shields.io/crates/d/kord.svg)](https://crates.io/crates/kord)
[![GitHub all releases](https://img.shields.io/github/downloads/twitchax/kord/total?label=binary)](https://github.com/twitchax/kord/releases)
[![Documentation](https://docs.rs/kord/badge.svg)](https://docs.rs/kord)
[![Rust](https://img.shields.io/badge/rust-nightly-blue.svg?maxAge=3600)](https://github.com/twitchax/kord)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# kord

A music theory binary and library for Rust.

## Binary Usage

### Install

Windows:

```powershell
iwr https://github.com/twitchax/kord/releases/latest/download/kord_x86_64-pc-windows-gnu.zip
Expand-Archive kord_x86_64-pc-windows-gnu.zip -DestinationPath C:\Users\%USERNAME%\AppData\Local\Programs\kord
```

Mac OS (Apple Silicon):

```bash
curl -LO https://github.com/twitchax/kord/releases/latest/download/kord_aarch64-apple-darwin.zip
unzip kord_aarch64-apple-darwin.zip -d /usr/local/bin
chmod a+x /usr/local/bin/kord
```

Linux:

```bash
curl -LO https://github.com/twitchax/kord/releases/latest/download/kord_x86_64-unknown-linux-gnu.zip
unzip kord_x86_64-unknown-linux-gnu.zip -d /usr/local/bin
chmod a+x /usr/local/bin/kord
```

Cargo:

```bash
$ cargo install kord
```

### Help Docs

```bash
$ kord -h

A tool to easily explore music theory principles.

Usage: kord.exe [COMMAND]

Commands:
  describe  Describes a chord
  play      Describes and plays a chord
  guess     Attempt to guess the chord from a set of notes (ordered by simplicity)
  help      Print this message or the help of the given subcommand(s)

Options:
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

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
kord = "*" #choose a version
```

### Examples

```rust
use klib::known_chord::KnownChord;
use klib::modifier::Degree;
use klib::note::*;
use klib::chord::*;

// Check to see what _kind_ of chord this is.
assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));
```

```rust
use crate::klib::base::Parsable;
use klib::note::*;
use klib::chord::*;

// Parse a chord from a string, and inspect the scale.
assert_eq!(Chord::parse("Cm7b5").unwrap().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);
```

```rust
use klib::note::*;
use klib::chord::*;

// From a note, create a chord, and look at the chord tones.
assert_eq!(C.into_chord().augmented().major7().chord(), vec![C, E, GSharp, B]);
```

## Test

```bash
cargo test
```

## License

MIT