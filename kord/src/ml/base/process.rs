//! Helpers for transforming full-song datasets into sanitized training samples.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use hound::{SampleFormat, WavReader};
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};

use crate::analyze::base::{get_frequency_space, get_smoothed_frequency_space};
use crate::core::{
    base::Res,
    note::{HasNoteId, Note},
};

use super::{helpers::save_kord_item, KordItem, FREQUENCY_SPACE_SIZE};

const DEFAULT_MIN_NOTE_FRACTION: f64 = 0.35;
const DEFAULT_MIN_NOTES: usize = 1;
const DEFAULT_MAX_NOTES: usize = 6;
const DEFAULT_MIN_DURATION_SECONDS: f64 = 0.0;

/// Options that control how song processing sanitizes measures into training samples.
///
/// These knobs influence how measures are translated into labels and audio buffers without
/// suppressing any measures. They can be used to smooth away extremely transient notes or to
/// cap how many notes participate in the final chord mask, but every measure is always emitted.
#[derive(Clone, Debug)]
pub struct SongProcessingOptions {
    /// The minimum fraction of a measure that a note must sound to be considered part of the chord.
    pub min_note_fraction: f64,
    /// The minimum number of distinct notes required for a measure to be considered chordal.
    pub min_notes: usize,
    /// The maximum number of notes to include from a measure (after sorting by prominence).
    pub max_notes: usize,
    /// Minimum audio duration required (in seconds) for a measure.
    pub min_duration_seconds: f64,
    /// Maximum number of samples to emit. When `None`, all qualifying measures are emitted.
    pub max_samples: Option<usize>,
}

impl Default for SongProcessingOptions {
    fn default() -> Self {
        Self {
            min_note_fraction: DEFAULT_MIN_NOTE_FRACTION,
            min_notes: DEFAULT_MIN_NOTES,
            max_notes: DEFAULT_MAX_NOTES,
            min_duration_seconds: DEFAULT_MIN_DURATION_SECONDS,
            max_samples: None,
        }
    }
}

/// Tracking container that carries aggregated information about a single measure while the
/// MIDI file is parsed. Every measure knows the ticks spanned, the note durations observed,
/// and its relative index in the song.
#[derive(Clone, Debug)]
struct MeasureInfo {
    index: u64,
    start_tick: u64,
    end_tick: u64,
    note_ticks: HashMap<u8, u64>,
}

/// Result returned from parsing a MIDI file.
///
/// Abstracts the complex tuple type used throughout this module so callers don't need to
/// reason about the concrete tuple shape.
#[derive(Clone, Debug)]
struct ParseMidiResult {
    tempo_events: Vec<(u64, u32)>,
    measures: HashMap<u64, MeasureInfo>,
}

/// Process a paired MIDI + WAV file into sanitized training samples saved on disk.
///
/// Every measure in the MIDI file yields a sample. The measure's aligned audio segment is
/// zero-padded to the next whole second, and that duration is encoded into the emitted file
/// name (`*_{seconds}s_*`) alongside the originating measure index and chord tones so that
/// downstream tooling can reason about sample length without reopening the binary payload.
pub fn process_song_samples(destination: impl AsRef<Path>, midi_path: impl AsRef<Path>, audio_path: impl AsRef<Path>, options: SongProcessingOptions) -> Res<Vec<PathBuf>> {
    let destination = destination.as_ref();
    fs::create_dir_all(destination)?;

    let midi_bytes = fs::read(&midi_path)?;
    let smf = Smf::parse(&midi_bytes)?;
    let ppq = match smf.header.timing {
        Timing::Metrical(t) => t.as_int(),
        _ => {
            return Err(anyhow::Error::msg("Only metrical MIDI files are supported."));
        }
    };

    let ParseMidiResult { tempo_events, measures } = parse_midi(&smf, ppq)?;

    let (audio_data, sample_rate) = load_wav_mono(&audio_path)?;
    let total_audio_seconds = audio_data.len() as f64 / sample_rate as f64;

    let mut saved_paths = Vec::new();
    let mut sorted_measures: Vec<_> = measures.into_values().collect();
    sorted_measures.sort_by_key(|m| m.index);

    let midi_stem = midi_path.as_ref().file_stem().and_then(|s| s.to_str()).unwrap_or("song");

    for measure in sorted_measures {
        if let Some(limit) = options.max_samples {
            if saved_paths.len() >= limit {
                break;
            }
        }

        let total_ticks = (measure.end_tick - measure.start_tick).max(1) as f64;
        let mut note_fractions = measure.note_ticks.iter().map(|(note, ticks)| (*note, *ticks as f64 / total_ticks)).collect::<Vec<_>>();
        note_fractions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut selected_notes: Vec<u8> = note_fractions.iter().filter(|(_, fraction)| *fraction >= options.min_note_fraction).map(|(note, _)| *note).collect();

        if selected_notes.len() < options.min_notes {
            for (note, _) in note_fractions.iter() {
                if selected_notes.contains(note) {
                    continue;
                }

                selected_notes.push(*note);

                if selected_notes.len() >= options.min_notes {
                    break;
                }
            }
        }

        if selected_notes.is_empty() && !note_fractions.is_empty() {
            selected_notes = note_fractions.iter().map(|(note, _)| *note).collect();
        }

        if selected_notes.len() > options.max_notes {
            selected_notes.truncate(options.max_notes);
        }

        let mut chord_notes: Vec<Note> = selected_notes.iter().map(|note| Note::try_from_midi(*note)).collect::<Res<_>>()?;

        chord_notes.sort_by_key(|n| n.id_index());

        let start_seconds = ticks_to_seconds(measure.start_tick, &tempo_events, ppq);
        let end_seconds = ticks_to_seconds(measure.end_tick, &tempo_events, ppq);

        if end_seconds <= start_seconds {
            continue;
        }

        if end_seconds > total_audio_seconds {
            // The audio does not contain this full measure.
            continue;
        }

        let duration_seconds = end_seconds - start_seconds;
        if duration_seconds < options.min_duration_seconds {
            continue;
        }

        let length_in_seconds = duration_seconds.ceil().clamp(1.0, u8::MAX as f64) as u8;
        let target_samples = length_in_seconds as usize * sample_rate as usize;

        let start_sample = (start_seconds * sample_rate as f64).floor() as usize;
        let end_sample = (end_seconds * sample_rate as f64).ceil() as usize;
        if start_sample >= audio_data.len() {
            break;
        }

        let mut buffer = vec![0.0f32; target_samples];
        let available_end = end_sample.min(audio_data.len());
        let available_samples = available_end.saturating_sub(start_sample);
        buffer[..available_samples].copy_from_slice(&audio_data[start_sample..available_end]);

        if available_samples == 0 {
            continue;
        }

        let frequency_space = get_frequency_space(&buffer, length_in_seconds);
        let smoothed = get_smoothed_frequency_space(&frequency_space, length_in_seconds);

        let mut spectrum = [0f32; FREQUENCY_SPACE_SIZE];
        for (index, (_, magnitude)) in smoothed.into_iter().enumerate().take(FREQUENCY_SPACE_SIZE) {
            spectrum[index] = magnitude;
        }

        let label = if chord_notes.is_empty() { 0 } else { Note::id_mask(&chord_notes) };
        let note_names = if chord_notes.is_empty() {
            "rest".to_string()
        } else {
            chord_notes.iter().map(ToString::to_string).collect::<Vec<_>>().join("_")
        };

        let item = KordItem {
            path: destination.to_path_buf(),
            frequency_space: spectrum,
            label,
        };

        let prefix = format!("{}_measure_{:04}_{}s_", midi_stem, measure.index, length_in_seconds);
        let path = save_kord_item(destination, &prefix, &note_names, &item)?;
        saved_paths.push(path);
    }

    if saved_paths.is_empty() {
        anyhow::bail!(
            "No qualifying measures were found when processing {} and {}.",
            midi_path.as_ref().display(),
            audio_path.as_ref().display()
        );
    }

    Ok(saved_paths)
}

/// Parse the MIDI file into tempo events and per-measure note statistics.
///
/// The parser keeps track of tempo and time-signature meta events so that later conversions
/// to wall-clock time can account for rubato sections or meter changes.
fn parse_midi(smf: &Smf<'_>, ppq: u16) -> Res<ParseMidiResult> {
    let mut tempo_events = vec![(0u64, 500_000u32)];
    let mut time_signature_numerator = 4u8;
    let mut time_signature_denominator = 4u32;
    let mut measures: HashMap<u64, MeasureInfo> = HashMap::new();
    let mut note_starts: HashMap<u8, Vec<u64>> = HashMap::new();
    let mut max_tick = 0u64;

    for track in &smf.tracks {
        let mut tick_accumulator = 0u64;
        for event in track {
            tick_accumulator += event.delta.as_int() as u64;
            max_tick = max_tick.max(tick_accumulator);

            match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(value)) => {
                    tempo_events.push((tick_accumulator, value.as_int()));
                }
                TrackEventKind::Meta(MetaMessage::TimeSignature(numerator, denominator, ..)) => {
                    let computed_denominator = 1u32 << (denominator as u32);
                    if time_signature_numerator != numerator || time_signature_denominator != computed_denominator {
                        time_signature_numerator = numerator;
                        time_signature_denominator = computed_denominator;
                    }
                }
                TrackEventKind::Midi { channel, message } => {
                    if channel.as_int() == 9 {
                        // General MIDI reserves channel 10 (index 9) for percussion, which should not
                        // contribute to harmonic labeling.
                        continue;
                    }

                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            if vel.as_int() > 0 {
                                note_starts.entry(key.as_int()).or_default().push(tick_accumulator);
                            } else {
                                register_note_off(
                                    key.as_int(),
                                    tick_accumulator,
                                    &mut note_starts,
                                    &mut measures,
                                    ppq,
                                    time_signature_numerator,
                                    time_signature_denominator,
                                );
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            register_note_off(
                                key.as_int(),
                                tick_accumulator,
                                &mut note_starts,
                                &mut measures,
                                ppq,
                                time_signature_numerator,
                                time_signature_denominator,
                            );
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    for (note, starts) in note_starts.iter_mut() {
        for start in starts.drain(..) {
            insert_note_segment(*note, start, max_tick, &mut measures, ppq, time_signature_numerator, time_signature_denominator);
        }
    }

    tempo_events.sort_by_key(|(tick, _)| *tick);
    tempo_events.dedup_by(|a, b| {
        if a.0 == b.0 {
            b.1 = a.1;
            true
        } else {
            false
        }
    });

    Ok(ParseMidiResult { tempo_events, measures })
}

/// Handle a MIDI note-off (or zero-velocity note-on) by recording the note duration inside
/// the appropriate measure bucket.
fn register_note_off(note: u8, tick: u64, note_starts: &mut HashMap<u8, Vec<u64>>, measures: &mut HashMap<u64, MeasureInfo>, ppq: u16, numerator: u8, denominator: u32) {
    if let Some(starts) = note_starts.get_mut(&note) {
        if let Some(start_tick) = starts.pop() {
            insert_note_segment(note, start_tick, tick, measures, ppq, numerator, denominator);
        }
    }
}

/// Partition a note segment across measure boundaries and accumulate the tick counts for
/// each slice. This ensures notes that span multiple bars contribute proportionally to every
/// measure they occupy.
fn insert_note_segment(note: u8, start_tick: u64, end_tick: u64, measures: &mut HashMap<u64, MeasureInfo>, ppq: u16, numerator: u8, denominator: u32) {
    if end_tick <= start_tick {
        return;
    }

    let measure_ticks = compute_measure_ticks(ppq, numerator, denominator);

    let mut current_start = start_tick;
    while current_start < end_tick {
        let measure_index = current_start / measure_ticks;
        let measure_start_tick = measure_index * measure_ticks;
        let measure_end_tick = (measure_index + 1) * measure_ticks;
        let segment_end = end_tick.min(measure_end_tick);
        let duration = segment_end.saturating_sub(current_start);

        let entry = measures.entry(measure_index).or_insert_with(|| MeasureInfo {
            index: measure_index,
            start_tick: measure_start_tick,
            end_tick: measure_end_tick,
            note_ticks: HashMap::new(),
        });

        *entry.note_ticks.entry(note).or_insert(0) += duration;
        entry.end_tick = entry.end_tick.max(segment_end);

        current_start = segment_end;
    }
}

/// Compute the number of ticks contained in a single measure for the current meter.
fn compute_measure_ticks(ppq: u16, numerator: u8, denominator: u32) -> u64 {
    let ppq = ppq as u64;
    let numerator = numerator as u64;
    let denominator = denominator as u64;
    // Each measure contains `numerator` beats, where each beat represents a `denominator` note.
    (ppq * numerator * 4) / denominator.max(1)
}

/// Load a WAV file into a mono floating-point buffer, normalizing integer sample formats to
/// `[-1.0, 1.0]` in the process.
fn load_wav_mono(path: impl AsRef<Path>) -> Res<(Vec<f32>, u32)> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let channels = spec.channels as usize;

    let raw_samples = match spec.sample_format {
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.map_err(anyhow::Error::from)).collect::<Res<Vec<_>>>()?,
        SampleFormat::Int => {
            if spec.bits_per_sample <= 16 {
                reader
                    .samples::<i16>()
                    .map(|s| s.map(|v| v as f32 / i16::MAX as f32).map_err(anyhow::Error::from))
                    .collect::<Res<Vec<_>>>()?
            } else {
                reader
                    .samples::<i32>()
                    .map(|s| s.map(|v| v as f32 / i32::MAX as f32).map_err(anyhow::Error::from))
                    .collect::<Res<Vec<_>>>()?
            }
        }
    };

    if channels == 0 {
        anyhow::bail!("Audio file has zero channels.");
    }

    let mut mono = Vec::with_capacity(raw_samples.len() / channels);
    for chunk in raw_samples.chunks(channels) {
        if chunk.is_empty() {
            continue;
        }
        let sum: f32 = chunk.iter().sum();
        mono.push(sum / channels as f32);
    }

    Ok((mono, sample_rate))
}

/// Convert an absolute tick offset into seconds using the precomputed tempo changes.
fn ticks_to_seconds(ticks: u64, tempo_events: &[(u64, u32)], ppq: u16) -> f64 {
    let mut elapsed = 0f64;
    let mut last_tick = 0u64;
    let mut current_tempo = tempo_events.first().map(|(_, tempo)| *tempo).unwrap_or(500_000);

    for &(event_tick, tempo) in tempo_events.iter().skip(1) {
        if event_tick >= ticks {
            break;
        }

        elapsed += (event_tick - last_tick) as f64 * seconds_per_tick(ppq, current_tempo);
        last_tick = event_tick;
        current_tempo = tempo;
    }

    elapsed + (ticks - last_tick) as f64 * seconds_per_tick(ppq, current_tempo)
}

/// Compute the duration of a single tick given the current tempo (microseconds per quarter
/// note) and pulses-per-quarter resolution.
fn seconds_per_tick(ppq: u16, tempo: u32) -> f64 {
    (tempo as f64 / 1_000_000.0) / ppq as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pitch::HasFrequency;
    use crate::ml::base::helpers::load_kord_item;
    use std::f32::consts::PI;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    /// Integration test that exercises real project fixtures end-to-end, ensuring the
    /// processor can read, sanitize, and persist multiple samples from realistic material.
    #[test]
    fn test_process_song_samples_with_repo_fixtures() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let midi_path = manifest_dir.join("tests/test.mid");
        let audio_path = manifest_dir.join("tests/test.wav");

        assert!(midi_path.exists(), "Fixture MIDI file missing at {:?}", midi_path);
        assert!(audio_path.exists(), "Fixture audio file missing at {:?}", audio_path);

        let destination = manifest_dir.join(".hidden/test_data/process_song");
        if destination.exists() {
            fs::remove_dir_all(&destination).unwrap();
        }
        fs::create_dir_all(&destination).unwrap();

        let options = SongProcessingOptions {
            max_samples: Some(64),
            ..Default::default()
        };

        let outputs = process_song_samples(&destination, &midi_path, &audio_path, options).expect("processing fixtures");
        assert!(!outputs.is_empty(), "Expected at least one processed sample");

        for output in outputs {
            assert!(output.starts_with(&destination), "Output {output:?} not in destination {destination:?}");
            assert!(output.exists(), "Missing output file {output:?}");

            let filename = output.file_name().and_then(|name| name.to_str()).expect("Output filename should be valid UTF-8");
            let _seconds_segment = filename
                .split('_')
                .find(|segment| segment.ends_with('s') && segment.trim_end_matches('s').chars().all(|c| c.is_ascii_digit()) && !segment.trim_end_matches('s').is_empty())
                .expect("Processed filename should include a seconds segment");

            let item = load_kord_item(&output);
            assert_ne!(item.label, 0, "Processed sample should have a non-zero chord label");
            assert!(item.frequency_space.iter().any(|&v| v != 0.0), "Frequency space should contain data");
        }
    }

    /// Synthetic test that constructs a single-measure MIDI/WAV pair and verifies the
    /// resulting filename and chord label are computed deterministically.
    #[test]
    fn test_process_song_samples_creates_expected_output() {
        let dir = tempdir().unwrap();
        let midi_path = dir.path().join("test.mid");
        let audio_path = dir.path().join("test.wav");
        let destination = dir.path().join("samples");

        write_test_midi(&midi_path).unwrap();
        write_test_wav(&audio_path, 2.0).unwrap();

        let options = SongProcessingOptions {
            max_samples: Some(4),
            ..Default::default()
        };

        let outputs = process_song_samples(&destination, &midi_path, &audio_path, options).unwrap();
        assert_eq!(outputs.len(), 1);

        let filename = outputs[0].file_name().and_then(|name| name.to_str()).expect("Output filename should be valid UTF-8");
        assert!(filename.contains("_2s_"), "Expected filename to include approximate duration (\"_2s_\"), found {filename:?}");

        let item = load_kord_item(&outputs[0]);
        let expected_notes = vec![Note::try_from_midi(60).unwrap(), Note::try_from_midi(64).unwrap(), Note::try_from_midi(67).unwrap()];

        assert_eq!(item.label, Note::id_mask(&expected_notes));
    }

    /// Even when the caller specifies thresholds that cannot be satisfied by the underlying
    /// material, every measure should still be emitted with the best available note set.
    #[test]
    fn test_process_song_samples_emits_measure_with_strict_thresholds() {
        let dir = tempdir().unwrap();
        let midi_path = dir.path().join("strict.mid");
        let audio_path = dir.path().join("strict.wav");
        let destination = dir.path().join("samples");

        write_test_midi(&midi_path).unwrap();
        write_test_wav(&audio_path, 2.0).unwrap();

        let options = SongProcessingOptions {
            max_samples: Some(4),
            min_note_fraction: 1.1,
            min_notes: 5,
            ..Default::default()
        };

        let outputs = process_song_samples(&destination, &midi_path, &audio_path, options).unwrap();
        assert_eq!(outputs.len(), 1);

        let item = load_kord_item(&outputs[0]);
        let expected_notes = vec![Note::try_from_midi(60).unwrap(), Note::try_from_midi(64).unwrap(), Note::try_from_midi(67).unwrap()];

        assert_eq!(item.label, Note::id_mask(&expected_notes));
    }

    /// Ensure percussion-channel notes do not influence chord labeling when processing samples.
    #[test]
    fn test_process_song_samples_ignores_percussion_notes() {
        let dir = tempdir().unwrap();
        let midi_path = dir.path().join("test_percussion.mid");
        let audio_path = dir.path().join("test.wav");
        let destination = dir.path().join("samples");

        write_test_midi_with_percussion(&midi_path).unwrap();
        write_test_wav(&audio_path, 2.0).unwrap();

        let options = SongProcessingOptions {
            max_samples: Some(4),
            ..Default::default()
        };

        let outputs = process_song_samples(&destination, &midi_path, &audio_path, options).unwrap();
        assert_eq!(outputs.len(), 1);

        let item = load_kord_item(&outputs[0]);
        let expected_notes = vec![Note::try_from_midi(60).unwrap(), Note::try_from_midi(64).unwrap(), Note::try_from_midi(67).unwrap()];
        assert_eq!(item.label, Note::id_mask(&expected_notes));

        let percussion_mask = Note::id_mask(&[Note::try_from_midi(36).unwrap(), Note::try_from_midi(38).unwrap()]);
        assert_eq!(item.label & percussion_mask, 0, "Percussion notes should be ignored when labeling samples");
    }

    /// Emit a minimal 4/4 MIDI file that plays a single C major triad for one measure.
    fn write_test_midi(path: &Path) -> Res<()> {
        write_test_midi_internal(path, false)
    }

    /// Emit the same MIDI file as [`write_test_midi`], but with an additional percussion track on channel 10.
    fn write_test_midi_with_percussion(path: &Path) -> Res<()> {
        write_test_midi_internal(path, true)
    }

    fn write_test_midi_internal(path: &Path, include_percussion: bool) -> Res<()> {
        let mut data = Vec::new();
        data.extend_from_slice(b"MThd");
        data.extend_from_slice(&6u32.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        let track_count = if include_percussion { 3u16 } else { 2u16 };
        data.extend_from_slice(&track_count.to_be_bytes());
        data.extend_from_slice(&480u16.to_be_bytes());

        let mut track0 = Vec::new();
        push_vlq(&mut track0, 0);
        track0.extend_from_slice(&[0xFF, 0x58, 0x04, 0x04, 0x02, 0x18, 0x08]);
        push_vlq(&mut track0, 0);
        track0.extend_from_slice(&[0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20]);
        push_vlq(&mut track0, 0);
        track0.extend_from_slice(&[0xFF, 0x2F, 0x00]);

        data.extend_from_slice(b"MTrk");
        data.extend_from_slice(&(track0.len() as u32).to_be_bytes());
        data.extend_from_slice(&track0);

        let mut track1 = Vec::new();
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0x90, 0x3C, 0x64]);
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0x90, 0x40, 0x64]);
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0x90, 0x43, 0x64]);
        push_vlq(&mut track1, 1920);
        track1.extend_from_slice(&[0x80, 0x3C, 0x40]);
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0x80, 0x40, 0x40]);
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0x80, 0x43, 0x40]);
        push_vlq(&mut track1, 0);
        track1.extend_from_slice(&[0xFF, 0x2F, 0x00]);

        data.extend_from_slice(b"MTrk");
        data.extend_from_slice(&(track1.len() as u32).to_be_bytes());
        data.extend_from_slice(&track1);

        if include_percussion {
            let mut percussion_track = Vec::new();
            push_vlq(&mut percussion_track, 0);
            percussion_track.extend_from_slice(&[0x99, 0x24, 0x64]); // Kick drum on channel 10.
            push_vlq(&mut percussion_track, 240);
            percussion_track.extend_from_slice(&[0x89, 0x24, 0x40]);
            push_vlq(&mut percussion_track, 0);
            percussion_track.extend_from_slice(&[0x99, 0x26, 0x64]); // Snare drum.
            push_vlq(&mut percussion_track, 240);
            percussion_track.extend_from_slice(&[0x89, 0x26, 0x40]);
            push_vlq(&mut percussion_track, 0);
            percussion_track.extend_from_slice(&[0xFF, 0x2F, 0x00]);

            data.extend_from_slice(b"MTrk");
            data.extend_from_slice(&(percussion_track.len() as u32).to_be_bytes());
            data.extend_from_slice(&percussion_track);
        }

        std::fs::write(path, data)?;
        Ok(())
    }

    /// Render a synthetic WAV file containing the same pitches as the MIDI fixture for a
    /// specified number of seconds.
    fn write_test_wav(path: &Path, seconds: f32) -> Res<()> {
        let sample_rate = 44_100u32;
        let total_samples = (sample_rate as f32 * seconds) as usize;
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;
        let frequencies = vec![
            Note::try_from_midi(60).unwrap().frequency(),
            Note::try_from_midi(64).unwrap().frequency(),
            Note::try_from_midi(67).unwrap().frequency(),
        ];

        for sample_index in 0..total_samples {
            let t = sample_index as f32 / sample_rate as f32;
            let mut value = 0.0f32;
            for freq in &frequencies {
                value += (2.0 * PI * freq * t).sin();
            }
            value /= frequencies.len() as f32;
            let scaled = (value * 0.4 * i16::MAX as f32) as i16;
            writer.write_sample(scaled)?;
        }

        writer.finalize()?;
        Ok(())
    }

    /// Helper that encodes an integer using the MIDI variable-length quantity format.
    fn push_vlq(buffer: &mut Vec<u8>, mut value: u32) {
        let mut bytes = [0u8; 4];
        let mut count = 0;
        loop {
            bytes[count] = (value & 0x7F) as u8;
            count += 1;
            value >>= 7;
            if value == 0 {
                break;
            }
        }

        for index in (0..count).rev() {
            let mut byte = bytes[index];
            if index != 0 {
                byte |= 0x80;
            }
            buffer.push(byte);
        }
    }
}
