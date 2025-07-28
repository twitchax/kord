//! Functions for analyzing audio files.

use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
    thread::sleep,
    time::Duration,
};

use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamBuilder, Sink, Source};

use crate::core::{base::Res, note::Note};

use super::base::get_notes_from_audio_data;

/// Retrieve a list of notes which are guessed from the given audio clip.
pub fn get_notes_from_audio_file(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<Vec<Note>> {
    let (data, length_in_seconds) = get_audio_data_from_file(file, start, end)?;

    get_notes_from_audio_data(&data, length_in_seconds)
}

/// Gets the audio data from a file.
pub fn get_audio_data_from_file(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<(Vec<f32>, u8)> {
    let path = file.as_ref();
    let start = start.unwrap_or_default();

    let decoder = Decoder::new(File::open(path)?)?.skip_duration(start);

    let num_channels = decoder.channels();
    let sample_rate = decoder.sample_rate();
    let samples: Vec<_> = if let Some(end) = end { decoder.take_duration(end - start).collect() } else { decoder.collect() };

    let num_samples = samples.len();

    let length_in_seconds = ((num_samples as f32) / (sample_rate as f32 * num_channels as f32)) as u8;

    // Cut the samples to the nearest second.
    let data = samples[..(length_in_seconds as f32 * sample_rate as f32 * num_channels as f32) as usize].to_vec();

    Ok((data, length_in_seconds))
}

/// Play the given segment of an audio file. Used to preview a clip before guessing notes from it.
#[coverage(off)]
pub fn preview_audio_file_clip(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<()> {
    let file = File::open(file)?;
    preview_audio_clip(file, start, end)
}

/// Play the given segment of an audio stream. Used to preview a clip before guessing notes from it.
#[coverage(off)]
pub fn preview_audio_clip(stream: impl Read + Seek + Send + Sync + 'static, start: Option<Duration>, end: Option<Duration>) -> Res<()> {
    let start = start.unwrap_or_default();
    let decoder = Decoder::new(stream)?.skip_duration(start);

    let stream = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream.mixer());

    if let Some(end) = end {
        stream.mixer().add(decoder.take_duration(end - start));
        sink.play();
        sleep(end - start);
    } else if let Some(duration) = decoder.total_duration() {
        stream.mixer().add(decoder);
        sink.play();
        sleep(duration);
    } else {
        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate() as f32;
        let samples: Vec<_> = decoder.collect();

        let time = Duration::from_secs((samples.len() as f32 / sample_rate).ceil() as u64);

        stream.mixer().add(SamplesBuffer::new(channels, sample_rate as u32, samples));
        sink.play();

        sleep(time);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::core::{base::Parsable, chord::Chord};

    use super::*;

    #[ignore]
    #[test]
    fn test_preview_audio_clip() {
        preview_audio_file_clip("tests/C7b9.wav", None, None).unwrap();
    }

    #[cfg(feature = "analyze_file")]
    #[test]
    fn test_get_notes_from_audio_file() {
        let notes = get_notes_from_audio_file("tests/C7b9.wav", None, None).unwrap();

        assert_eq!(Chord::parse("C7b9").unwrap(), Chord::try_from_notes(&notes).unwrap()[0]);
    }

    #[cfg(feature = "analyze_file")]
    #[cfg(feature = "analyze_file_mp3")]
    #[test]
    fn test_get_notes_from_mp3_file() {
        let notes = get_notes_from_audio_file("tests/C7b9.mp3", None, None).unwrap();

        assert_eq!(Chord::parse("C7b9").unwrap(), Chord::try_from_notes(&notes).unwrap()[0]);
    }
}
