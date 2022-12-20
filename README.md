[![Build and Test](https://github.com/twitchax/kord/actions/workflows/build.yml/badge.svg)](https://github.com/twitchax/kord/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/kord.svg)](https://crates.io/crates/kord)
[![crates.io](https://img.shields.io/crates/d/kord.svg)](https://crates.io/crates/kord)
[![Documentation](https://docs.rs/kord/badge.svg)](https://docs.rs/kord)
[![Rust](https://img.shields.io/badge/rust-nightly-blue.svg?maxAge=3600)](https://github.com/twitchax/kord)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# kord

A music theory binary and library for Rust.

## Binary Usage

### Install

Windows:

```powershell
iwr https://github.com/twitchax/kord/releases/download/v0.1.1/kord_x86_64-pc-windows-gnu.zip
Expand-Archive kord_x86_64-pc-windows-gnu.zip -DestinationPath C:\Users\%USERNAME%\AppData\Local\Programs\kord
```

Mac OS (Apple Silicon):

```bash
curl -LO https://github.com/twitchax/kord/releases/download/v0.1.1/kord_aarch64-apple-darwin.zip
unzip kord_aarch64-apple-darwin.zip -d /usr/local/bin
chmod a+x /usr/local/bin/kord
```

Linux:

```bash
curl -LO https://github.com/twitchax/kord/releases/download/v0.1.1/kord_x86_64-unknown-linux-gnu.zip
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

Usage: kord [COMMAND]

Commands:
  describe  Describes a chord
  play      Describes and plays a chord
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

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
kord = "0.1"
```

### Examples

```rust
use klib::known_chord::KnownChord;
use klib::modifier::Degree;
use klib::note::*;
use klib::chord::*;

// Check to see what _kind_ of chord this is.
assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));

// Parse a chord from a string, and inspect the scale.
assert_eq!(Chord::parse("Cm7b5").unwrap().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);

// From a note, create a chord, and look at the chord tones.
assert_eq!(C.into_chord().augmented().major7().chord(), vec![C, E, GSharp, B]);
```

## Test

```bash
cargo test
```

## License

MIT