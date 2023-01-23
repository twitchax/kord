use std::{fs::File, path::Path, thread::sleep, time::Duration};

use anyhow::anyhow;
use rodio::{buffer::SamplesBuffer, Decoder, OutputStream, Source};

use crate::{base::Res, note::Note};

pub fn get_notes_from_audio_file(file: impl AsRef<Path>, start: Duration, end: Duration) -> Res<Vec<Note>> {
    Ok(vec![])
}

pub fn preview_audio_clip(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<()> {
    let path = file.as_ref();
    let file = File::open(path)?;
    let start = start.unwrap_or_default();
    let decoder = Decoder::new(file)?;
    let decoder = decoder.skip_duration(start).convert_samples();
    let (_stream, stream_handle) = OutputStream::try_default()?;
    if let Some(end) = end {
        stream_handle.play_raw(decoder.take_duration(end - start))?;
        sleep(end - start);
    } else if let Some(duration) = decoder.total_duration() {
        stream_handle.play_raw(decoder)?;
        sleep(duration);
    } else {
        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate() as f32;
        let samples: Vec<_> = decoder.collect();
        let time = Duration::from_secs((samples.len() as f32 / sample_rate).ceil() as u64);
        stream_handle.play_raw(SamplesBuffer::new(channels, sample_rate as u32, samples))?;
        sleep(time);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_preview_audio_clip() {
        preview_audio_clip(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/Am.flac"), None, None).unwrap();
    }

    #[test]
    #[cfg(feature = "all_audio")]
    fn test_preview_mp3_clip() {
        preview_audio_clip(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/Am.mp3"), None, None).unwrap();
    }
}
