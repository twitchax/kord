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
        /// The source directory for the gathered samples.
        #[arg(long, default_value = ".hidden/samples")]
        source: String,

        /// The destination directory for the trained model.
        #[arg(long, default_value = "model")]
        destination: String,

        /// The log directory for training.
        #[arg(long, default_value = ".hidden/train_log")]
        log: String,

        /// Simulation data set size.
        #[arg(long, default_value_t = 100)]
        simulation_size: usize,

        /// The device to use for training.
        #[arg(long, default_value = "gpu")]
        device: String,

        /// The number of Multi Layer Perceptron (MLP) layers.
        #[arg(long, default_value_t = 0)]
        mlp_layers: usize,

        /// The number of neurons in each Multi Layer Perceptron (MLP) layer.
        #[arg(long, default_value_t = 1024)]
        mlp_size: usize,

        /// The Multi Layer Perceptron (MLP) dropout rate.
        #[arg(long, default_value_t = 0.3)]
        mlp_dropout: f64,

        /// The number of epochs to train for.
        #[arg(long, default_value_t = 32)]
        model_epochs: usize,

        /// The number of samples to use per epoch.
        #[arg(long, default_value_t = 100)]
        model_batch_size: usize,

        /// The number of workers to use for training.
        #[arg(long, default_value_t = 32)]
        model_workers: usize,

        /// The seed used for training.
        #[arg(long, default_value_t = 76980)]
        model_seed: u64,

        /// The Adam optimizer learning rate.
        #[arg(long, default_value_t = 1e-4)]
        adam_learning_rate: f64,

        /// The Adam optimizer weight decay.
        #[arg(long, default_value_t = 5e-5)]
        adam_weight_decay: f64,

        /// The Adam optimizer beta1.
        #[arg(long, default_value_t = 0.9)]
        adam_beta1: f32,

        /// The Adam optimizer beta2.
        #[arg(long, default_value_t = 0.999)]
        adam_beta2: f32,

        /// The Adam optimizer epsilon.
        #[arg(long, default_value_t = f32::EPSILON)]
        adam_epsilon: f32,

        /// The "sigmoid strength" of the final pass.
        #[arg(long, default_value_t = 1.0)]
        sigmoid_strength: f32,
    },

    /// Records audio from the microphone, and using the trained model, guesses the chord.
    #[cfg(all(feature = "ml_infer", feature = "analyze_mic"))]
    Infer {
        /// Sets the duration of listening time (in seconds).
        #[arg(short, long, default_value_t = 10)]
        length: u8,
    },

    /// Plots the kord item from the specified source file.
    #[cfg(feature = "plot")]
    Plot {
        /// The source file to plot.
        source: String,

        /// The minimum frequency value of the plot.
        #[arg(long, default_value_t = 0.0)]
        x_min: f32,

        /// The maximum frequency value of the plot.
        #[arg(long, default_value_t = 8192.0)]
        x_max: f32,
    },

    /// Generates simulated audio samples for the specified notes.
    #[cfg(feature = "ml_train")]
    GenerateSamples {
        /// The destination directory for the generated samples.
        #[arg(long, default_value = ".hidden/samples")]
        destination: String,

        /// The prefix of the generated sample files.
        #[arg(long, default_value = "0_")]
        prefix: String,

        /// The radius of the generated note peaks.
        #[arg(long, default_value_t = 5.0)]
        peak_radius: f32,

        /// The harmonic decay of the generated note peaks (how much each harmonic is decayed from the previous).
        #[arg(long, default_value_t = 0.5)]
        harmonic_decay: f32,

        /// The amount of "frequency wobble" in the generated note peaks.
        #[arg(long, default_value_t = 1.0)]
        frequency_wobble: f32,
    },
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
        #[cfg(any(feature = "analyze_mic", feature = "analyze_file"))]
        Some(Command::Analyze { analyze_command }) => match analyze_command {
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
        },
        #[cfg(any(feature = "ml", feature = "ml_base", feature = "ml_train", feature = "ml_infer"))]
        Some(Command::Ml { ml_command }) => match ml_command {
            #[cfg(feature = "ml_train")]
            Some(MlCommand::Gather { destination, length }) => {
                klib::ml::base::gather::gather_sample(&destination, length)?;
            }
            #[cfg(feature = "ml_train")]
            Some(MlCommand::Train {
                source,
                destination,
                log,
                simulation_size,
                device,
                mlp_layers,
                mlp_size,
                mlp_dropout,
                model_epochs,
                model_batch_size,
                model_workers,
                model_seed,
                adam_learning_rate,
                adam_weight_decay,
                adam_beta1,
                adam_beta2,
                adam_epsilon,
                sigmoid_strength,
            }) => {
                use burn_autodiff::ADBackendDecorator;
                use klib::ml::base::TrainConfig;

                let config = TrainConfig {
                    source,
                    destination,
                    log,
                    simulation_size,
                    mlp_layers,
                    mlp_size,
                    mlp_dropout,
                    model_epochs,
                    model_batch_size,
                    model_workers,
                    model_seed,
                    adam_learning_rate,
                    adam_weight_decay,
                    adam_beta1,
                    adam_beta2,
                    adam_epsilon,
                    sigmoid_strength,
                };

                match device.as_str() {
                    #[cfg(feature = "ml_gpu")]
                    "gpu" => {
                        use burn_tch::{TchBackend, TchDevice};

                        let device = TchDevice::Cuda(0);

                        klib::ml::train::run_training::<ADBackendDecorator<TchBackend<f32>>>(device, &config, true)?;
                    }
                    "cpu" => {
                        use burn_ndarray::{NdArrayBackend, NdArrayDevice};

                        let device = NdArrayDevice::Cpu;

                        klib::ml::train::run_training::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config, true)?;
                    }
                    _ => {
                        return Err(anyhow::Error::msg("Invalid device (must choose either `gpu` [requires `ml_gpu` feature] or `cpu`)."));
                    }
                }
            }
            #[cfg(all(feature = "ml_infer", feature = "analyze_mic"))]
            Some(MlCommand::Infer { length }) => {
                use burn_ndarray::{NdArrayBackend, NdArrayDevice};
                use klib::ml::{
                    base::{KordItem, FREQUENCY_SPACE_SIZE},
                    infer::run_inference,
                };

                // Prepare the audio data.
                let audio_data = futures::executor::block_on(klib::analyze::mic::get_audio_data_from_microphone(length))?;
                let frequency_space = klib::analyze::base::get_frequency_space(&audio_data, length);
                let smoothed_frequency_space: [_; FREQUENCY_SPACE_SIZE] = klib::analyze::base::get_smoothed_frequency_space(&frequency_space, length)
                    .into_iter()
                    .take(FREQUENCY_SPACE_SIZE)
                    .map(|(_, v)| v)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let kord_item = KordItem {
                    frequency_space: smoothed_frequency_space,
                    ..Default::default()
                };

                let device = NdArrayDevice::Cpu;

                // Run the inference.
                let notes = run_inference::<NdArrayBackend<f32>>(&device, &kord_item)?;

                // Show the results.
                show_notes_and_chords(&notes)?;
            }
            #[cfg(feature = "plot")]
            Some(MlCommand::Plot { source, x_min, x_max }) => {
                use anyhow::Context;
                use klib::{
                    analyze::base::translate_frequency_space_to_peak_space,
                    helpers::plot_frequency_space,
                    ml::{
                        base::MEL_SPACE_SIZE,
                        train::helpers::{load_kord_item, mel_filter_banks_from},
                    },
                };

                let kord_item = load_kord_item(&source);

                let path = std::path::Path::new(&source);
                let name = path.file_name().context("Could not get file name.")?.to_str().context("Could not map file name to str.")?;

                // Plot frequency space.
                let frequency_file_name = format!("{}_frequency", name);
                let frequency_space = kord_item.frequency_space.into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&frequency_space, "KordItem Frequency Space", &frequency_file_name, x_min, x_max);

                // Plot mel space.
                let mel_file_name = format!("{}_mel", name);
                let mel_space = mel_filter_banks_from(&kord_item.frequency_space)
                    .into_iter()
                    .enumerate()
                    .map(|(k, v)| (k as f32, v))
                    .collect::<Vec<_>>();
                plot_frequency_space(&mel_space, "KordItem Mel Space", &mel_file_name, 0.0, MEL_SPACE_SIZE as f32);

                // Plot peak space.
                let peak_file_name = format!("{}_peak", name);
                let mut peak_space = translate_frequency_space_to_peak_space(&frequency_space);
                peak_space.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                peak_space.iter_mut().skip(12).for_each(|(_, v)| *v = 0.0);
                peak_space.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
                plot_frequency_space(&peak_space, "KordItem Peak Space", &peak_file_name, x_min, x_max);

                // Plot mel peak space.
                let mel_peak_file_name = format!("{}_mel_peak", name);
                let peak_space = peak_space.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
                let mel_peak_space = mel_filter_banks_from(&peak_space).into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&mel_peak_space, "KordItem Mel Peak Space", &mel_peak_file_name, 0.0, MEL_SPACE_SIZE as f32);

                // Log Frequency Space
                let log_file_name = format!("{}_log", name);
                let log_frequency_space = kord_item.frequency_space.into_iter().map(|v| v.log2()).collect::<Vec<_>>();
                plot_frequency_space(
                    &log_frequency_space.iter().enumerate().map(|(k, v)| (k as f32, *v)).collect::<Vec<_>>(),
                    "KordItem Log Space",
                    &log_file_name,
                    x_min,
                    x_max,
                );

                // Plot time space.
                let harmonic_file_name = format!("{}_time", name);
                let time_space = klib::analyze::base::get_time_space(&peak_space);
                plot_frequency_space(&time_space, "KordItem Time Space", &harmonic_file_name, x_min, x_max);
            }
            #[cfg(feature = "ml_train")]
            Some(MlCommand::GenerateSamples {
                destination,
                prefix,
                peak_radius,
                harmonic_decay,
                frequency_wobble,
            }) => {
                use klib::{
                    core::{interval::Interval, note::ALL_PITCH_NOTES},
                    ml::{base::helpers::save_kord_item, train::helpers::get_simulated_kord_item},
                };

                for _ in 0..10 {
                    for note in ALL_PITCH_NOTES.iter().skip(12).take(74) {
                        let note = *note;

                        for k in 0..4 {
                            let mut notes = vec![note];

                            match k {
                                0 => {}
                                1 => {
                                    notes.push(note + Interval::MajorThird);
                                }
                                2 => {
                                    notes.push(note + Interval::MajorThird);
                                    notes.push(note + Interval::PerfectFifth);
                                }
                                3 => {
                                    notes.push(note + Interval::MajorThird);
                                    notes.push(note + Interval::PerfectFifth);
                                    notes.push(note + Interval::MajorSeventh);
                                }
                                _ => unreachable!(),
                            }

                            notes.sort();
                            let note_names = notes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("_");

                            // Generate the sample.
                            let kord_item = get_simulated_kord_item(&notes, peak_radius, harmonic_decay, frequency_wobble);

                            // Save the sample.
                            save_kord_item(&destination, &prefix, &note_names, &kord_item)?;
                        }
                    }
                }
            }
            None => {
                return Err(anyhow::Error::msg("No subcommand given for `ml`."));
            }
        },
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
