use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
    thread::sleep,
    time::Duration,
};

use rodio::{buffer::SamplesBuffer, Decoder, OutputStream, Source};

use crate::{base::Res, listen::get_notes_from_audio_data, note::Note};

/// Retrieve a list of notes which are guessed from the given audio clip.
pub fn get_notes_from_audio_file(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<Vec<Note>> {
    let path = file.as_ref();
    let file = File::open(path)?;
    let start = start.unwrap_or_default();
    let decoder = Decoder::new(file)?;
    let decoder = decoder.skip_duration(start).convert_samples();
    let sample_rate = decoder.sample_rate() as usize;
    let samples: Vec<_> = if let Some(end) = end { decoder.take_duration(end - start).collect() } else { decoder.collect() };
    let clip_length = sample_rate.div_ceil(samples.len()).try_into()?;
    get_notes_from_audio_data(&samples, clip_length)
}

/// Play the given segment of an audio file. Used to preview a clip before guessing notes from it.
pub fn preview_audio_file_clip(file: impl AsRef<Path>, start: Option<Duration>, end: Option<Duration>) -> Res<()> {
    let file = File::open(file)?;
    preview_audio_clip(file, start, end)
}

/// Play the given segment of an audio stream. Used to preview a clip before guessing notes from it.
pub fn preview_audio_clip(stream: impl Read + Seek + Send + Sync + 'static, start: Option<Duration>, end: Option<Duration>) -> Res<()> {
    let start = start.unwrap_or_default();
    let decoder = Decoder::new(stream)?;
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

    use crate::note::{BFive, CFive, DSharpSeven, DSix, GEight};

    use super::*;

    #[test]
    fn test_preview_audio_clip() {
        preview_audio_file_clip(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/MSC_AnaSaurus[shrt]-Cmaj7-9.wav"), None, None).unwrap();
    }

    #[test]
    fn test_get_notes_from_audio_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/MSC_AnaSaurus[shrt]-Cmaj7-9.wav");
        let notes = get_notes_from_audio_file(path, None, None).expect("notes");
        println!("{notes:?}");
        assert!(notes.contains(&CFive));
        assert!(notes.contains(&DSharpSeven));
        assert!(notes.contains(&GEight));
        assert!(notes.contains(&BFive));
        assert!(notes.contains(&DSix));
    }
}
