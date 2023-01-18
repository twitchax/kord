use std::{
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::base::Res;

#[cfg(feature = "audio")]
pub fn preview_audio_file_segment(
    file: impl AsRef<Path>,
    start: Option<String>,
    end: Option<String>,
) -> Res<()> {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(file.as_ref());
    if let Some(start) = start {
        cmd.arg("-ss").arg(start);
    }
    if let Some(end) = end {
        cmd.arg("-to").arg(end);
    }
    let mut convert = cmd
        .args(["-f", "wav", "pipe:1"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut player = Command::new("ffplay")
        .args("-f wav -i pipe:0 -autoexit".split(' '))
        .stdin(Stdio::piped())
        .spawn()?;
    let player_input = player
        .stdin
        .as_mut()
        .expect("stdin specified but not available? bug?");
    let convert_output = convert
        .stdout
        .as_mut()
        .expect("stdin specified but not available? bug?");

    let mut buf = [0u8; 0x1000];
    loop {
        let n = convert_output.read(&mut buf)?;
        if n == 0 {
            break;
        }
        player_input.write_all(&buf)?;
    }
    convert.wait()?.exit_ok()?;
    player.wait()?.exit_ok()?;

    Ok(())
}
