//! Module to help samples from microphone and save them to disk.

use std::path::Path;

use crate::{
    analyze::{
        base::{get_frequency_space, get_smoothed_frequency_space}
    },
    core::{
        base::{Parsable, Void},
        note::{HasNoteId, Note},
    },
    ml::base::{KordItem, FREQUENCY_SPACE_SIZE},
};

use crate::analyze::mic::get_audio_data_from_microphone;

use super::helpers::save_kord_item;

/// Gather a sample from the microphone and save it to disk.
#[no_coverage]
pub fn gather_sample(destination: impl AsRef<Path>, length_in_seconds: u8) -> Void {
    println!("Listening ...");

    let audio_data = futures::executor::block_on(get_audio_data_from_microphone(length_in_seconds))?;
    let frequency_space = get_frequency_space(&audio_data, length_in_seconds).into_iter().collect::<Vec<_>>();
    let smoothed_frequency_space = get_smoothed_frequency_space(&frequency_space, length_in_seconds).into_iter().take(FREQUENCY_SPACE_SIZE);

    let mut line = String::new();
    println!("Enter notes: ");
    let _ = std::io::stdin().read_line(&mut line).unwrap();

    let notes = line.split(' ').into_iter().filter(|s| !s.is_empty()).map(Note::parse).collect::<Result<Vec<_>, _>>()?;
    let note_names = notes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("_");

    let mut label: u128 = 0;

    for note in notes {
        label |= note.id();
    }

    let item = KordItem {
        path: destination.as_ref().to_owned(),
        frequency_space: smoothed_frequency_space.into_iter().map(|(_, v)| v).collect::<Vec<_>>().try_into().unwrap(),
        label,
    };

    save_kord_item(destination, "", &note_names, &item)?;

    Ok(())
}
