//! Base types for the machine learning train module.

use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Cursor, Write},
    path::Path,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{core::base::Void, ml::base::{KordItem, FREQUENCY_SPACE_SIZE}};

/// Load the kord sample from the binary file into a new [`KordItem`].
pub(crate) fn load_kord_item(path: &std::path::Path) -> KordItem {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // Read 8192 f32s in big endian from the file.
    let mut frequency_space = [0f32; 8192];

    (0usize..FREQUENCY_SPACE_SIZE).for_each(|k| {
        frequency_space[k] = reader.read_f32::<BigEndian>().unwrap();
    });

    let label = reader.read_u128::<BigEndian>().unwrap();

    KordItem { frequency_space, label }
}

/// Save the kord sample into a binary file.
pub(crate) fn save_kord_item(destination: impl AsRef<Path>, note_names: &str, item: &KordItem) -> Void {
    let mut output_data: Vec<u8> = Vec::with_capacity(FREQUENCY_SPACE_SIZE);
    let mut cursor = Cursor::new(&mut output_data);

    // Write frequency space.
    for value in item.frequency_space {
        cursor.write_f32::<BigEndian>(value)?;
    }

    // Write result.
    cursor.write_u128::<BigEndian>(item.label)?;

    // Get the hash.
    let mut hasher = DefaultHasher::new();
    output_data.hash(&mut hasher);
    let hash = hasher.finish();

    // Write the file.
    let mut f = File::create(destination.as_ref().join(format!("{}_{}.bin", note_names, hash)))?;
    f.write_all(&output_data)?;

    Ok(())
}
