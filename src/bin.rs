use std::{time::Duration};

use clap::{Parser, Subcommand};
use klib::{base::Void, chord::{Chordable, HasChord, Chord, Parsable}, pitch::HasFrequency, octave::Octave};
use rodio::{OutputStream, Sink, source::SineWave, Source};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Describes a chord
    Describe {
        /// Chord symbol to parse
        symbol: String,

        /// Sets the octave of the primary note
        #[arg(short, long, default_value_t = 4i8)]
        octave: i8,
    },

    /// Describes and plays a chord
    Play {
        /// Chord symbol to parse
        symbol: String,

        /// Sets the octave of the primary note
        #[arg(short, long, default_value_t = 4i8)]
        octave: i8,

        /// Sets the delay between notes (in seconds)
        #[arg(short, long, default_value_t = 0.2f32)]
        delay: f32,

        /// Sets the duration of play (in seconds)
        #[arg(short, long, default_value_t = 3.0f32)]
        length: f32,
    },
}

fn main() -> Void {
    let args = Args::parse();

    match args.command {
        Some(Commands::Describe { symbol, octave }) => {
            let chord = Chord::parse(&symbol)?.with_octave(Octave::Zero + octave);

            describe(&chord);
        }
        Some(Commands::Play { symbol, octave, delay, length }) => {
            let chord = Chord::parse(&symbol)?.with_octave(Octave::Zero + octave);

            play(&chord, delay, length);
        }
        None => {
            println!("No command given.");
        }
    }
    Ok(())
}

fn describe(chord: &Chord) {
    println!("{}", chord);
}

fn play(chord: &Chord, delay: f32, length: f32) {
    describe(chord);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut sinks = vec![];

    for (k, n) in chord.chord().into_iter().enumerate() {
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(n.frequency())
            .take_duration(Duration::from_secs_f32(length - k as f32 * delay))
            .delay(Duration::from_secs_f32(k as f32 * delay))
            .amplify(0.20);

        sink.append(source);

        sinks.push(sink);
    }

    std::thread::sleep(Duration::from_secs_f32(length));
}