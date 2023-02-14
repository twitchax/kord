use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use klib::core::{
    base::{Parsable, Res, Void},
    chord::{Chord, Chordable},
    note::Note,
    octave::Octave,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Describes a chord.
    ///
    /// The `symbol` has some special syntax.  These are the parts:
    ///
    /// * The root note (e.g., `C`, `D#`, `Eb`, `F##`, `Gbb`, `A♯`, `B♭`, etc.).
    ///
    /// * Any modifiers (e.g., `7`, `9`, `m7b5`, `sus4`, `dim`, `+`, `maj7`, `-maj7`, `m7b5#9`, etc.).
    ///
    /// * Any extensions (e.g., `add9`, `add11`, `add13`, `add2`, etc.).
    ///
    /// * Zero or one slash notes (e.g., `/E`, `/G#`, `/Fb`, etc.).
    ///
    /// * Zero or one octaves for the root (default is 4), using the `@` symbol (e.g., `@4`, `@5`, `@6`, etc.).
    ///
    /// * Zero or one inversions (e.g., `^1`, `^2`, `^3`, etc.).
    ///
    /// * Zero or one "crunchy" modifiers, which moves "higher notes" into the same octave frame as the root (i.e., `!`).
    Describe {
        /// Chord symbol to parse.
        symbol: String,

        /// Sets the octave of the primary note.
        #[arg(short, long, default_value_t = 4i8)]
        octave: i8,
    },

    /// Describes and plays a chord.
    ///
    /// Please see `describe` for more information on the chord symbol syntax.
    Play {
        /// Chord symbol to parse.
        symbol: String,

        /// Sets the delay between notes (in seconds).
        #[arg(short, long, default_value_t = 0.2f32)]
        delay: f32,

        /// Sets the duration of play (in seconds).
        #[arg(short, long, default_value_t = 3.0f32)]
        length: f32,

        /// Fade in duration (in seconds).
        #[arg(short, long, default_value_t = 0.1f32)]
        fade_in: f32,
    },

    /// Loops on a set of chord changes, while simultaneously outputting the descriptions.
    Loop {
        /// Chord symbol to parse, followed by length in 32nd notes (e.g., "Cm7|32 Dm7|32 Em7|32").
        ///
        /// If no length is given, the default is 32.
        chords: Vec<String>,

        /// Sets the beats per minute of the playback loop.
        #[arg(short, long, default_value_t = 60f32)]
        bpm: f32,
    },

    /// Attempt to guess the chord from a set of notes (ordered by simplicity).
    Guess {
        /// A set of notes from which the guesser will attempt to build a chord.
        notes: Vec<String>,
    },

    /// Set of commands to analyze audio data.
    #[cfg(any(feature = "analyze", feature = "analyze_base", feature = "analyze_mic", feature = "analyze_file"))]
    Analyze {
        #[command(subcommand)]
        analyze_command: Option<AnalyzeCommand>,
    },

    /// Set of commands to train and infer with ML.
    #[cfg(any(feature = "ml", feature = "ml_base", feature = "ml_train", feature = "ml_infer"))]
    Ml {
        #[command(subcommand)]
        ml_command: Option<MlCommand>,
    },
}

#[derive(Subcommand, Debug)]
enum AnalyzeCommand {
    /// Records audio from the microphone, and guesses pitches / chords.
    #[cfg(feature = "analyze_mic")]
    Mic {
        /// Sets the duration of listening time (in seconds).
        #[arg(short, long, default_value_t = 10)]
        length: u8,
    },

    /// Guess pitches and chords from the specified section of an audio file.
    #[cfg(feature = "analyze_file")]
    File {
        /// Whether or not to play a preview of the selected section of the
        /// audio file before analyzing.
        #[arg(long = "no-preview", action=ArgAction::SetFalse, default_value_t = true)]
        preview: bool,
        /// How far into the file to begin analyzing, as understood by systemd.time(7)
        #[arg(short, long)]
        start_time: Option<String>,
        /// How far into the file to stop analyzing, as understood by systemd.time(7)
        #[arg(short, long)]
        end_time: Option<String>,
        /// The source file to listen to/analyze.
        source: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum MlCommand {
    /// Records audio from the microphone, and writes the resulting sample to disk.
    #[cfg(feature = "ml_train")]
    Gather {
        /// Sets the destination directory for the gathered samples.
        #[arg(short, long, default_value = ".hidden/samples")]
        destination: String,

        /// Sets the duration of listening time (in seconds).
        #[arg(short, long, default_value_t = 10)]
        length: u8,
    },

    /// Runs the ML trainer using burn-rs, tch-rs, and CUDA as defaults.
    #[cfg(feature = "ml_train")]
    Train {
        
    },

    /// Records audio from the microphone, and using the trained model, guesses the chord.
    #[cfg(feature = "ml_infer")]
    Infer {

    }
}

fn main() -> Void {
    let args = Args::parse();

    start(args)?;

    Ok(())
}

fn start(args: Args) -> Void {
    match args.command {
        Some(Command::Describe { symbol, octave }) => {
            let chord = Chord::parse(&symbol)?.with_octave(Octave::Zero + octave);

            describe(&chord);
        }
        Some(Command::Play { symbol, delay, length, fade_in }) => {
            let chord = Chord::parse(&symbol)?;

            play(&chord, delay, length, fade_in)?;
        }
        Some(Command::Guess { notes }) => {
            // Parse the notes.
            let notes = notes.into_iter().map(|n| Note::parse(&n)).collect::<Result<Vec<_>, _>>()?;

            // Get the chord from the notes.
            let candidates = Chord::from_notes(&notes)?;

            for candidate in candidates {
                describe(&candidate);
            }
        }
        Some(Command::Loop { chords, bpm }) => {
            let chord_pairs = chords
                .into_iter()
                .map(|c| {
                    let mut parts = c.split('|');

                    let chord = Chord::parse(parts.next().unwrap()).unwrap();

                    let length = parts.next().map(|l| l.parse::<u16>().unwrap()).unwrap_or(32);

                    (chord, length)
                })
                .collect::<Vec<_>>();

            loop {
                for (chord, length) in chord_pairs.iter() {
                    let length = (*length as f32) * 60f32 / bpm / 8f32;
                    play(chord, 0.0, length, 0.1)?;
                }
            }
        }
        #[cfg(any(feature = "analyze", feature = "analyze_base", feature = "analyze_mic", feature = "analyze_file"))]
        Some(Command::Analyze { analyze_command }) => {
            match analyze_command {
                #[cfg(feature = "analyze_mic")]
                Some(AnalyzeCommand::Mic { length }) => {
                    let notes = futures::executor::block_on(Note::from_mic(length))?;

                    show_notes_and_chords(&notes)?;
                }
                #[cfg(feature = "analyze_file")]
                Some(AnalyzeCommand::File { preview, start_time, end_time, source }) => {
                    use klib::analyze::file::{get_notes_from_audio_file, preview_audio_file_clip};

                    let start_time = if let Some(t) = start_time { Some(parse_duration0::parse(&t)?) } else { None };
                    let end_time = if let Some(t) = end_time { Some(parse_duration0::parse(&t)?) } else { None };
                    if preview {
                        preview_audio_file_clip(&source, start_time, end_time)?;
                    }
                    let notes = get_notes_from_audio_file(&source, start_time, end_time)?;
                    show_notes_and_chords(&notes)?;
                }
                None => {
                    return Err(anyhow::Error::msg("No subcommand given for `analyze`."));
                }
            }
        }
        #[cfg(any(feature = "ml", feature = "ml_base", feature = "ml_train", feature = "ml_infer"))]
        Some(Command::Ml { ml_command }) => {
            match ml_command {
                #[cfg(feature = "ml_train")]
                Some(MlCommand::Gather { destination, length }) => {
                    use klib::ml::train::gather::gather_sample;

                    gather_sample(&destination, length)?;
                }
                #[cfg(feature = "ml_train")]
                Some(MlCommand::Train {}) => {
                    unimplemented!();
                }
                #[cfg(feature = "ml_infer")]
                Some(MlCommand::Infer {}) => {
                    unimplemented!();
                }
                None => {
                    return Err(anyhow::Error::msg("No subcommand given for `ml`."));
                }
            }
        }
        None => {
            return Err(anyhow::Error::msg("No command given."));
        }
    }
    Ok(())
}

fn describe(chord: &Chord) {
    println!("{}", chord);
}

fn play(chord: &Chord, delay: f32, length: f32, fade_in: f32) -> Void {
    describe(chord);

    #[cfg(feature = "audio")]
    {
        use klib::core::base::Playable;
        use std::time::Duration;

        let _playable = chord.play(delay, length, fade_in)?;
        std::thread::sleep(Duration::from_secs_f32(length));
    }

    Ok(())
}

fn show_notes_and_chords(notes: &[Note]) -> Res<()> {
    println!("Notes: {}", notes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));

    let candidates = Chord::from_notes(notes)?;

    if candidates.is_empty() {
        println!("No chord candidates found");
    } else {
        for candidate in candidates {
            describe(&candidate);
        }
    }
    Ok(())
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        start(Args {
            command: Some(Command::Describe {
                symbol: "Cmaj7b9@3^2!".to_string(),
                octave: 4,
            }),
        })
        .unwrap();
    }

    #[test]
    fn test_guess() {
        start(Args {
            command: Some(Command::Guess {
                notes: vec!["C".to_owned(), "E".to_owned(), "G".to_owned()],
            }),
        })
        .unwrap();
    }
}
