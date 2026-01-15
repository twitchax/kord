#![recursion_limit = "256"]

#[cfg(any(feature = "analyze_file", feature = "ml_sample_process"))]
use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use klib::core::{
    base::{Parsable, Res, Void},
    chord::{Chord, Chordable},
    note::Note,
    octave::Octave,
};
use tracing_subscriber::{filter::LevelFilter, fmt::SubscriberBuilder};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag that specifies verbose logging.
    #[arg(short, long, conflicts_with = "quiet")]
    verbose: bool,

    /// Flag that suppresses all tracing output.
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,

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
    #[cfg(feature = "analyze_base")]
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
        #[arg(long = "no-preview", action = ArgAction::SetFalse, default_value_t = true)]
        preview: bool,

        /// How far into the file to begin analyzing, as understood by systemd.time(7).
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
    #[cfg(feature = "ml_sample_gather")]
    Gather {
        /// Sets the destination directory for the gathered samples.
        #[arg(short, long, default_value = ".hidden/samples")]
        destination: String,

        /// Sets the duration of listening time (in seconds).
        #[arg(short, long, default_value_t = 10)]
        length: u8,
    },

    /// Processes paired MIDI and audio files into sanitized training samples.
    #[cfg(feature = "ml_sample_process")]
    Process {
        /// Sets the destination directory for the processed samples.
        #[arg(short, long, default_value = ".hidden/samples")]
        destination: String,

        /// The MIDI file containing chord annotations for the song.
        #[arg(long)]
        midi: PathBuf,

        /// The audio file (WAV or FLAC) aligned with the MIDI file.
        #[arg(long)]
        audio: PathBuf,

        /// Minimum fraction of a measure that a note must sound to be included in the chord.
        #[arg(long, default_value_t = 0.2)]
        min_fraction: f32,

        /// Minimum number of distinct notes required to emit a sample.
        #[arg(long, default_value_t = 1)]
        min_notes: usize,

        /// Maximum number of notes to retain after sorting by prominence.
        #[arg(long, default_value_t = 20)]
        max_notes: usize,

        /// Minimum duration (in seconds) required for the measure to be considered.
        #[arg(long, default_value_t = 1.0)]
        min_duration: f32,

        /// Maximum number of samples to emit.
        #[arg(long)]
        limit: Option<usize>,
    },

    /// Runs the ML trainer using burn-rs, tch-rs, and CUDA as defaults.
    #[cfg(feature = "ml_train")]
    Train {
        /// The noise asset root for the simulated data.
        #[arg(long, default_value = "kord/samples/noise")]
        noise_asset_root: String,

        /// The directories (or `sim`) to draw training samples from.
        #[arg(long = "training-sources", value_name = "PATH", required = true, num_args = 1..)]
        training_sources: Vec<String>,

        /// Optional directories (or `sim`) to draw validation samples from.
        #[arg(long = "validation-sources", value_name = "PATH", num_args = 1..)]
        validation_sources: Vec<String>,

        /// The destination directory for the trained model.
        #[arg(long, default_value = "kord/model")]
        destination: String,

        /// The log directory for training.
        #[arg(long, default_value = ".hidden/train_log")]
        log: String,

        /// The backend to use for training (`tch`, `cuda`, `wgpu`, `candle`, or `ndarray`).
        /// This usually requires that one of the backend compilation flags was set.
        ///
        /// You can also use a "remote" backend, such as `ws://localhost:3000` to connect to a remote server.
        #[arg(long, default_value = "tch")]
        backend: String,

        /// Simulation data set size.
        #[arg(long, default_value_t = 500)]
        simulation_size: usize,

        /// Simulation peak radius.
        #[arg(long, default_value_t = 2.0)]
        simulation_peak_radius: f32,

        /// Simulation harmonic decay.
        #[arg(long, default_value_t = 0.1)]
        simulation_harmonic_decay: f32,

        /// Simulation frequency wobble.
        #[arg(long, default_value_t = 0.2)]
        simulation_frequency_wobble: f32,

        /// The number of times to replicate captured samples during training.
        #[arg(long, default_value_t = 8)]
        captured_oversample_factor: usize,

        /// The number of Multi Head Attention (MHA) heads.
        #[arg(long, default_value_t = 16)]
        mha_heads: usize,

        /// Dropout rate applied to attention and trunk layers.
        #[arg(long, default_value_t = 0.2)]
        dropout: f64,

        /// The hidden size of the model's MLP trunk.
        ///
        /// The trunk is a lightweight MLP inserted between attention and the final output head.
        #[arg(long, default_value_t = 1024)]
        trunk_hidden_size: usize,

        /// The number of epochs to train for.
        #[arg(long, default_value_t = 16)]
        model_epochs: usize,

        /// The number of samples to use per epoch.
        #[arg(long, default_value_t = 100)]
        model_batch_size: usize,

        /// The number of workers to use for training.
        #[arg(long, default_value_t = 64)]
        model_workers: usize,

        /// The seed used for training.
        #[arg(long, default_value_t = 76980)]
        model_seed: u64,

        /// The Adam optimizer learning rate.
        #[arg(long, default_value_t = 1e-3)]
        adam_learning_rate: f64,

        /// The Adam optimizer weight decay.
        #[arg(long, default_value_t = 1e-4)]
        adam_weight_decay: f32,

        /// The Adam optimizer beta1.
        #[arg(long, default_value_t = 0.9)]
        adam_beta1: f32,

        /// The Adam optimizer beta2.
        #[arg(long, default_value_t = 0.999)]
        adam_beta2: f32,

        /// The Adam optimizer epsilon.
        #[arg(long, default_value_t = f32::EPSILON)]
        adam_epsilon: f32,

        /// Suppresses the training plots.
        #[arg(long, action=ArgAction::SetTrue, default_value_t = false)]
        no_plots: bool,
    },

    /// Records audio from the microphone, and using the trained model, guesses the chord.
    #[cfg(feature = "ml_infer")]
    Infer {
        #[command(subcommand)]
        infer_command: Option<InferCommand>,
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

    /// Runs the ML trainer across various hyperparameters, and outputs the results.
    #[cfg(feature = "ml_hpt")]
    Hpt {
        /// The source directory for the gathered samples.
        #[arg(long, default_value = "kord/samples")]
        source: String,

        /// The destination directory for the trained model.
        #[arg(long, default_value = "model")]
        destination: String,

        /// The log directory for training.
        #[arg(long, default_value = ".hidden/train_log")]
        log: String,

        /// The device to use for training.
        #[arg(long, default_value = "gpu")]
        device: String,
    },

    /// Runs a "training server" that can be used by an `ml train` command remotely.
    #[cfg(feature = "ml_server")]
    Serve {
        /// The port to bind the server to.
        #[arg(long, default_value_t = 3000)]
        port: u16,

        /// The backend to use for training (`cuda`, `wgpu`, `ndarray`).
        /// This usually requires that one of the backend compilation flags was set.
        #[arg(long, default_value = "wgpu")]
        backend: String,
    },
}

#[derive(Subcommand, Debug)]
enum InferCommand {
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

fn main() -> Void {
    let args = Args::parse();

    init_tracing(args.verbose, args.quiet);

    start(args)?;

    Ok(())
}

fn init_tracing(verbose: bool, quiet: bool) {
    let level_filter = if quiet {
        LevelFilter::OFF
    } else if verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    SubscriberBuilder::default()
        .with_ansi(true)
        .with_level(!quiet)
        .with_file(false)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_max_level(level_filter)
        .init();

    if quiet {
        return;
    }

    if verbose {
        tracing::debug!("Tracing initialized at DEBUG level");
    } else {
        tracing::info!("Tracing initialized at INFO level");
    }
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
            let candidates = Chord::try_from_notes(&notes)?;

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

                    let length = parts.next().map_or(32, |l| l.parse::<u16>().unwrap());

                    (chord, length)
                })
                .collect::<Vec<_>>();

            loop {
                for (chord, length) in &chord_pairs {
                    let length = (*length as f32) * 60f32 / bpm / 8f32;
                    play(chord, 0.0, length, 0.1)?;
                }
            }
        }
        #[cfg(feature = "analyze_base")]
        Some(Command::Analyze { analyze_command }) => match analyze_command {
            #[cfg(feature = "analyze_mic")]
            Some(AnalyzeCommand::Mic { length }) => {
                let notes = futures::executor::block_on(Note::try_from_mic(length))?;

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
        #[cfg(feature = "ml_base")]
        Some(Command::Ml { ml_command }) => match ml_command {
            #[cfg(feature = "ml_sample_gather")]
            Some(MlCommand::Gather { destination, length }) => {
                klib::ml::base::gather::gather_sample(destination, length)?;
            }
            #[cfg(feature = "ml_sample_process")]
            Some(MlCommand::Process {
                destination,
                midi,
                audio,
                min_fraction,
                min_notes,
                max_notes,
                min_duration,
                limit,
            }) => {
                use klib::ml::base::process::{process_song_samples, SongProcessingOptions};

                if max_notes < min_notes {
                    return Err(anyhow::Error::msg("`max-notes` must be greater than or equal to `min-notes`."));
                }

                if !(0.0..=1.0).contains(&min_fraction) {
                    return Err(anyhow::Error::msg("`min-fraction` must be between 0.0 and 1.0."));
                }

                let options = SongProcessingOptions {
                    min_note_fraction: min_fraction as f64,
                    min_notes,
                    max_notes,
                    min_duration_seconds: min_duration as f64,
                    max_samples: limit,
                };

                let paths = process_song_samples(&destination, midi, audio, options)?;
                println!("Generated {} samples.", paths.len());
            }
            #[cfg(feature = "ml_train")]
            Some(MlCommand::Train {
                noise_asset_root,
                training_sources,
                validation_sources,
                destination,
                log,
                simulation_size,
                backend,
                simulation_peak_radius,
                simulation_harmonic_decay,
                simulation_frequency_wobble,
                captured_oversample_factor,
                mha_heads,
                dropout,
                trunk_hidden_size,
                model_epochs,
                model_batch_size,
                model_workers,
                model_seed,
                adam_learning_rate,
                adam_weight_decay,
                adam_beta1,
                adam_beta2,
                adam_epsilon,
                no_plots,
            }) => {
                #[cfg(any(feature = "ml_tch", feature = "ml_cuda", feature = "ml_wgpu", feature = "ml_candle", feature = "ml_ndarray"))]
                use burn::backend::Autodiff;
                #[cfg(any(feature = "ml_tch", feature = "ml_cuda", feature = "ml_wgpu", feature = "ml_candle", feature = "ml_ndarray"))]
                use klib::ml::base::PrecisionElement;
                use klib::ml::base::TrainConfig;

                #[allow(unused_variables)]
                let config = TrainConfig {
                    noise_asset_root,
                    training_sources,
                    validation_sources,
                    destination,
                    log,
                    simulation_size,
                    simulation_peak_radius,
                    simulation_harmonic_decay,
                    simulation_frequency_wobble,
                    captured_oversample_factor,
                    mha_heads,
                    dropout,
                    trunk_hidden_size,
                    model_epochs,
                    model_batch_size,
                    model_workers,
                    model_seed,
                    adam_learning_rate,
                    adam_weight_decay,
                    adam_beta1,
                    adam_beta2,
                    adam_epsilon,
                    no_plots,
                };

                match backend.as_str() {
                    #[cfg(feature = "ml_tch")]
                    "tch" => {
                        #[cfg(not(target_os = "macos"))]
                        use burn::backend::libtorch::LibTorchDevice;
                        use burn::backend::LibTorch;

                        #[cfg(not(target_os = "macos"))]
                        let device = LibTorchDevice::Cuda(0);
                        #[cfg(target_os = "macos")]
                        let device = LibTorchDevice::Mps;

                        klib::ml::train::run_training::<Autodiff<LibTorch<PrecisionElement>>>(device, &config, true, true)?;
                    }
                    #[cfg(feature = "ml_cuda")]
                    "cuda" => {
                        use burn::backend::{cuda::CudaDevice, Cuda};

                        let device = CudaDevice::default();

                        klib::ml::train::run_training::<Autodiff<Cuda<PrecisionElement>>>(device, &config, true, true)?;
                    }
                    #[cfg(feature = "ml_wgpu")]
                    "wgpu" => {
                        use burn::backend::{wgpu::WgpuDevice, Wgpu};

                        let device = WgpuDevice::default();

                        klib::ml::train::run_training::<Autodiff<Wgpu<PrecisionElement>>>(device, &config, true, true)?;
                    }
                    #[cfg(feature = "ml_candle")]
                    "candle" => {
                        #[cfg(not(target_os = "macos"))]
                        use burn::backend::candle::CandleDevice;
                        use burn::backend::Candle;

                        #[cfg(not(target_os = "macos"))]
                        let device = CandleDevice::cuda(0);
                        #[cfg(target_os = "macos")]
                        let device = CandleDevice::Cpu;

                        klib::ml::train::run_training::<Autodiff<Candle<PrecisionElement>>>(device, &config, true, true)?;
                    }
                    #[cfg(feature = "ml_ndarray")]
                    "ndarray" => {
                        use burn::backend::{ndarray::NdArrayDevice, NdArray};

                        let device = NdArrayDevice::default();

                        klib::ml::train::run_training::<Autodiff<NdArray<PrecisionElement>>>(device, &config, true, true)?;
                    }
                    _ => {
                        #[cfg(feature = "ml_remote")]
                        if backend.starts_with("ws://") {
                            use burn::backend::{remote::RemoteDevice, RemoteBackend};

                            let device = RemoteDevice::new(&backend);
                            klib::ml::train::run_training::<Autodiff<RemoteBackend>>(device, &config, true, true)?;

                            return Ok(());
                        }

                        return Err(anyhow::Error::msg(
                            "Invalid backend (must choose either `tch` [requires `ml_tch` feature], `cuda` [requires `ml_cuda` feature], `wgpu` [requires `ml_wgpu` feature], `candle` [requires `ml_candle` feature], `ndarray` [requires `ml_ndarray` feature]), or a remote backend (requires `ml_remote` feature).",
                        ));
                    }
                };
            }
            #[cfg(feature = "ml_infer")]
            Some(MlCommand::Infer { infer_command }) => match infer_command {
                #[cfg(feature = "analyze_mic")]
                Some(InferCommand::Mic { length }) => {
                    use klib::ml::infer::infer;

                    // Prepare the audio data.
                    let audio_data = futures::executor::block_on(klib::analyze::mic::get_audio_data_from_microphone(length))?;

                    // Run the inference.
                    let result = infer(&audio_data, length)?;

                    // Show the results.
                    show_inference_result(&result)?;
                }
                #[cfg(feature = "analyze_file")]
                Some(InferCommand::File { preview, start_time, end_time, source }) => {
                    use klib::{
                        analyze::file::{get_audio_data_from_file, preview_audio_file_clip},
                        ml::infer::infer,
                    };

                    let start_time = if let Some(t) = start_time { Some(parse_duration0::parse(&t)?) } else { None };
                    let end_time = if let Some(t) = end_time { Some(parse_duration0::parse(&t)?) } else { None };

                    if preview {
                        preview_audio_file_clip(&source, start_time, end_time)?;
                    }

                    // Prepare the audio data.
                    let (audio_data, length) = get_audio_data_from_file(&source, start_time, end_time)?;

                    // Run inference.
                    let result = infer(&audio_data, length)?;

                    // Show the results.
                    show_inference_result(&result)?;
                }
                _ => {
                    return Err(anyhow::Error::msg("Invalid inference command."));
                }
            },
            #[cfg(feature = "plot")]
            Some(MlCommand::Plot { source, x_min, x_max }) => {
                use anyhow::Context;
                use klib::{
                    analyze::base::{compute_cqt, translate_frequency_space_to_peak_space},
                    helpers::plot_frequency_space,
                    ml::base::{
                        helpers::{fold_binary, harmonic_convolution, load_kord_item, mel_filter_banks_from, note_binned_convolution},
                        MEL_SPACE_SIZE,
                    },
                };

                let kord_item = load_kord_item(&source);

                let path = std::path::Path::new(&source);
                let name = path.file_name().context("Could not get file name.")?.to_str().context("Could not map file name to str.")?;

                // Plot frequency space.
                let frequency_file_name = format!("{}_frequency", name);
                let frequency_space = kord_item.frequency_space.into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&frequency_space, "KordItem Frequency Space", &frequency_file_name, x_min, x_max);

                // Plot harmonic convolution.
                let harmonic_file_name = format!("{}_harmonic", name);
                let harmonic_space = harmonic_convolution(&kord_item.frequency_space).into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&harmonic_space, "KordItem Harmonic Space", &harmonic_file_name, x_min, x_max);

                // Plot CQT space.
                let cqt_file_name = format!("{}_cqt", name);
                let cqt_space = compute_cqt(&kord_item.frequency_space).into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&cqt_space, "KordItem CQT Space", &cqt_file_name, 0.0, 256.0);

                // Plot note-binned convolution space.
                let convolution_file_name = format!("{}_convolution", name);
                let convolution_space = note_binned_convolution(&kord_item.frequency_space);
                let convolution_space_as_freq = convolution_space.iter().enumerate().map(|(k, v)| (k as f32, *v)).collect::<Vec<_>>();
                plot_frequency_space(&convolution_space_as_freq, "KordItem Note-Binned Convolution Space", &convolution_file_name, 0.0, 128.0);

                // Plot folded note-binned convolution space.
                let folded_convolution_file_name = format!("{}_convolution_folded", name);
                let folded_convolution_space = fold_binary(&convolution_space).into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();
                plot_frequency_space(&folded_convolution_space, "KordItem Folded Note-Binned Convolution Space", &folded_convolution_file_name, 0.0, 12.0);

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
            #[cfg(feature = "ml_hpt")]
            Some(MlCommand::Hpt { source, destination, log, device }) => {
                use klib::ml::train::execute::hyper_parameter_tuning;

                hyper_parameter_tuning(source, destination, log, device)?;
            }
            #[cfg(feature = "ml_server")]
            Some(MlCommand::Serve { port, backend }) => {
                match backend.as_str() {
                    #[cfg(feature = "ml_cuda")]
                    "cuda" => {
                        burn::server::start::<burn::backend::Cuda>(Default::default(), port);
                    }
                    #[cfg(feature = "ml_wgpu")]
                    "wgpu" => {
                        burn::server::start::<burn::backend::Wgpu>(Default::default(), port);
                    }
                    #[cfg(feature = "ml_ndarray")]
                    "ndarray" => {
                        burn::server::start::<burn::backend::NdArray>(Default::default(), port);
                    }
                    _ => {
                        return Err(anyhow::Error::msg(
                            "Invalid backend (must choose either `cuda` [requires `ml_cuda` feature], `wgpu` [requires `ml_wgpu` feature], or `ndarray` [requires `ml_ndarray` feature]).",
                        ));
                    }
                };
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
    println!("{}", chord.format_with_scale_candidates());
}

fn play(chord: &Chord, delay: f32, length: f32, fade_in: f32) -> Void {
    describe(chord);

    #[cfg(feature = "audio")]
    {
        use klib::core::base::Playable;
        use std::time::Duration;

        let _playable = chord.play(Duration::from_secs_f32(delay), Duration::from_secs_f32(length), Duration::from_secs_f32(fade_in))?;
        std::thread::sleep(Duration::from_secs_f32(length));
    }

    Ok(())
}

#[cfg(feature = "ml_infer")]
fn show_inference_result(result: &klib::ml::infer::InferenceResult) -> Res<()> {
    use klib::core::base::HasStaticName;

    // Show detected pitches.
    let pitch_names: Vec<String> = result.pitches.iter().map(|p| klib::core::named_pitch::NamedPitch::from(*p).static_name().to_string()).collect();
    println!("Pitches: {}", pitch_names.join(" "));

    // Show pitch deltas for debugging.
    println!("\nPitch deltas (probability - threshold):");
    let pitch_class_names = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
    for (i, &delta) in result.pitch_deltas.iter().enumerate() {
        println!("  {}: {:.3}", pitch_class_names[i], delta);
    }

    // Show chord candidates.
    if result.chords.is_empty() {
        println!("\nNo chord candidates found");
    } else {
        println!("\nChord candidates:");
        for candidate in &result.chords {
            describe(candidate);
        }
    }

    Ok(())
}

fn show_notes_and_chords(notes: &[Note]) -> Res<()> {
    println!("Notes: {}", notes.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "));

    let candidates = Chord::try_from_notes(notes)?;

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
            verbose: false,
            quiet: false,
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
            verbose: false,
            quiet: false,
            command: Some(Command::Guess {
                notes: vec!["C".to_owned(), "E".to_owned(), "G".to_owned()],
            }),
        })
        .unwrap();
    }
}
