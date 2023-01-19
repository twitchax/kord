use std::{
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;
use which::which;

use crate::{base::Res, debug, listen::get_notes_from_audio_data, note::Note};

/// Check to make sure `ffmpeg` is installed. If `preview` is true, also make
/// sure `ffplay` is installed.
pub fn check_dependencies(preview: bool) -> Res<()> {
    if let Err(err) = which("ffmpeg") {
        return Err(anyhow!(
            "error: {err}\nNote: file analysis requires ffmpeg to be installed https://ffmpeg.org/"
        ));
    }
    if preview {
        if let Err(err) = which("ffplay") {
            return Err(anyhow!("error: {err}\nNote: file analysis clip preview requires ffplay to be installed https://ffmpeg.org/"));
        }
    }
    Ok(())
}

/// Retrieve the notes used from an audio file. Files convertible by ffmpeg are
/// acceptable.
pub fn get_notes_from_audio_file(
    file: impl AsRef<Path>,
    start: Option<impl AsRef<str>>,
    end: Option<impl AsRef<str>>,
    ffmpeg_debug: bool,
) -> Res<Vec<Note>> {
    let mut cmd = Command::new("ffmpeg");
    // pass ffmpeg the file to decode
    cmd.arg("-i").arg(file.as_ref());
    if let Some(start) = start {
        // pass ffmpeg the start time
        cmd.arg("-ss").arg(start.as_ref());
    } // else, start at the beginning of the file
    if let Some(end) = end {
        // pass ffmpeg the end time
        cmd.arg("-to").arg(end.as_ref());
    } // else, continue to the end of the file
    if ffmpeg_debug {
        // ffmpeg outputs file information and errors to stderr, allow the
        // user to see that information if they need it.
        cmd.stderr(Stdio::inherit());
    }
    let output = cmd
        .args("-f f32le -c:a pcm_f32le -sample_rate 44100 pipe:1".split(' '))
        /* Flags here:
         *  -f f32le: format the output file as raw PCM 32-bit little-endian
         *            float values
         *  -c:a pcmf32le: set the output 'a'udio 'c'odec to the same
         *  -sample_rate 44100: Set a consistent sample rate for calculating
         *                      clip time from sample count
         *  pipe:1: write the output audio data to stdout
         */
        .output()?;
    #[cfg(feature = "debug")]
    let stdout_size = output.stdout.len();
    debug!("got {stdout_size} bytes from ffmpeg");
    let mut stdout = output.stdout.iter();
    let mut samples = vec![];
    // Take four bytes at a time
    while let (Some(a), Some(b), Some(c), Some(d)) =
        (stdout.next(), stdout.next(), stdout.next(), stdout.next())
    {
        // construct a f32 and push it to the list
        samples.push(f32::from_le_bytes([*a, *b, *c, *d]));
    }
    let sample_count = samples.len();
    debug!("got {sample_count} samples...");
    // Sample count / sample rate = time in seconds
    let length = (sample_count / 44100).try_into()?;
    debug!("...which comes out to {length} seconds");
    // pass the transcoded audio stream to the same function used to listen.
    get_notes_from_audio_data(&samples, length)
}

/// Play the specified clip of the given audio file.
pub fn preview_audio_file_segment(
    file: impl AsRef<Path>,
    start: Option<impl AsRef<str>>,
    end: Option<impl AsRef<str>>,
    ffmpeg_debug: bool,
) -> Res<()> {
    // ffplay lacks the `-to` option, so we need to pipe this through ffmpeg
    // to trim it then pass that output to ffplay.
    let mut cmd = Command::new("ffmpeg");
    // pass ffmpeg the file to decode
    cmd.arg("-i").arg(file.as_ref());
    if let Some(start) = start {
        // pass ffmpeg the start time
        cmd.arg("-ss").arg(start.as_ref());
    } // else, start at the beginning of the file
    if let Some(end) = end {
        // pass ffmpeg the end time
        cmd.arg("-to").arg(end.as_ref());
    } // else, continue to the end of the file
    if ffmpeg_debug {
        // ffmpeg outputs file information and errors to stderr, allow the
        // user to see that information if they need it.
        cmd.stderr(Stdio::inherit());
    }
    let mut convert = cmd
        .args(["-f", "wav", "pipe:1"])
        /*  flags here:
         * -f wav: output a wav file. Some format must be specified.
         * pipe:1: write the encoded audio to stdout
         */
        .stdout(Stdio::piped())
        .spawn()?;

    // this command plays back the audio
    let mut player = Command::new("ffplay");
    if ffmpeg_debug {
        player.stderr(Stdio::inherit());
    }
    let mut player = player
        .args("-f wav -i pipe:0 -autoexit".split(' '))
        /* Flags here:
         * -f wav: the input file is a wav. This must be specified because the
         *         input file doesn't have a name.
         * -i pipe:0: read audio data from stdin
         * -autoexit: without this ffplay never exits, with it, it exits when
         *            the file is done playing.
         */
        .stdin(Stdio::piped())
        .spawn()?;

    let player_input = player
        .stdin
        .as_mut()
        .expect("stdin specified but not available? bug?");
    let convert_output = convert
        .stdout
        .as_mut()
        .expect("stdout specified but not available? bug?");

    // arbitrary sized buffer for taking chunks from the transcode process to
    // the playback process
    let mut buf = [0u8; 0x1000];
    loop {
        // read a chunk from the transcode process
        let n = convert_output.read(&mut buf)?;
        if n == 0 {
            // no more data to read.
            break;
        }
        // write the chunk to the player.
        player_input.write_all(&buf[..=n])?;
    }

    convert.wait()?.exit_ok()?;
    player.wait()?.exit_ok()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::note::*;

    #[test]
    fn test_guess_notes() {
        let source_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/Am.mp3");
        match get_notes_from_audio_file(&source_file, None::<&str>, None::<&str>, false) {
            Ok(notes) => {
                assert!(notes.contains(&A));
                assert!(notes.contains(&C));
                assert!(notes.contains(&E));
            }
            Err(err) => panic!("failed to get notes from audio file: {err:?}"),
        }
    }
}
