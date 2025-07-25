//! Helpers for training models.

use std::path::Path;

use burn::{
    tensor::backend::{AutodiffBackend, Backend},
    train::{MultiLabelClassificationOutput, TrainOutput, TrainStep, ValidStep},
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    core::{
        interval::Interval,
        note::{HasNoteId, Note, ALL_PITCH_NOTES},
        pitch::HasFrequency,
    },
    ml::base::{helpers::load_kord_item, model::KordModel, KordItem, FREQUENCY_SPACE_SIZE},
};

use super::data::KordBatch;

// Classification adapters.

impl<B: AutodiffBackend> TrainStep<KordBatch<B>, MultiLabelClassificationOutput<B>> for KordModel<B> {
    fn step(&self, item: KordBatch<B>) -> TrainOutput<MultiLabelClassificationOutput<B>> {
        let item = self.forward_classification(item);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<KordBatch<B>, MultiLabelClassificationOutput<B>> for KordModel<B> {
    fn step(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        self.forward_classification(item)
    }
}

// Operations for simulating kord samples.

/// Create a simulated kord sample item from a noise basis and a semi-random collection of notes.
pub fn get_simulated_kord_item(noise_asset_root: impl AsRef<Path>, notes: &[Note], peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> KordItem {
    let noise_asset_root = noise_asset_root.as_ref().to_str().unwrap();
    let wobble_divisor = 35.0;

    let mut result = match get_random_between(0.0, 4.0).round() as u32 {
        0 | 4 => load_kord_item(format!("{noise_asset_root}/no_noise.bin")),
        1 => load_kord_item(format!("{noise_asset_root}/pink_noise.bin")),
        2 => load_kord_item(format!("{noise_asset_root}/white_noise.bin")),
        3 => load_kord_item(format!("{noise_asset_root}/brown_noise.bin")),
        _ => unreachable!(),
    };

    for note in notes {
        let mut harmonic_strength = 1.0;

        let note_frequency = note.frequency() * (1.0 + 1.0 / wobble_divisor * get_random_between(-frequency_wobble, frequency_wobble));

        let true_harmonic_series = (1..14)
            .map(|k| {
                let f = k as f32 * note_frequency;
                f * (1.0 + 1.0 / wobble_divisor * get_random_between(-frequency_wobble, frequency_wobble))
            })
            .collect::<Vec<_>>();

        for harmonic_frequency in true_harmonic_series {
            if harmonic_frequency - peak_radius < 0.0 || harmonic_frequency + peak_radius > FREQUENCY_SPACE_SIZE as f32 {
                continue;
            }

            let peak_strength = 4000.0 * harmonic_strength * get_random_between(0.8, 1.0);

            for i in (harmonic_frequency - peak_radius).round() as usize..(harmonic_frequency + peak_radius).round() as usize {
                result.frequency_space[i] += peak_strength * (1.0 - ((2.0 / peak_radius) * (i as f32 - harmonic_frequency).abs()).tanh());
            }

            harmonic_strength *= 1.0 - harmonic_decay;
        }
    }

    result.label = Note::id_mask(notes);

    result
}

/// Create simulated kord sample item by randomly selecting notes from a list of notes,
/// and use the given configuration.
pub fn get_simulated_kord_items<R>(noise_asset_root: R, count: usize, peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> Vec<KordItem>
where
    R: AsRef<Path> + Clone + Send + Sync,
{
    let results = (0..count).into_par_iter().map(move |_| {
        let note_count = 60;
        let chord_count = 5;
        let mut inner_result = Vec::with_capacity(note_count * chord_count);

        for note in ALL_PITCH_NOTES.iter().skip(24).take(note_count) {
            let note = *note;

            for k in 0..chord_count {
                let mut notes = vec![];

                match k {
                    0 => {
                        notes.push(note);
                    }
                    1 => {
                        notes.push(note);
                    }
                    2 => {
                        notes.push(note);
                        notes.push(note + get_random_item(&[Interval::MinorSecond, Interval::MajorSecond, Interval::MinorThird, Interval::MajorThird, Interval::PerfectFourth]));
                    }
                    3 => {
                        notes.push(note);
                        notes.push(note + get_random_item(&[Interval::MinorSecond, Interval::MajorSecond, Interval::MinorThird, Interval::MajorThird, Interval::PerfectFourth]));
                        notes.push(note + get_random_item(&[Interval::AugmentedFourth, Interval::PerfectFifth, Interval::AugmentedFifth, Interval::MajorSixth]));
                    }
                    4 => {
                        notes.push(note);
                        notes.push(note + get_random_item(&[Interval::MinorSecond, Interval::MajorSecond, Interval::MinorThird, Interval::MajorThird, Interval::PerfectFourth]));
                        notes.push(note + get_random_item(&[Interval::AugmentedFourth, Interval::PerfectFifth, Interval::AugmentedFifth, Interval::MajorSixth]));
                        notes.push(
                            note + get_random_item(&[
                                Interval::MinorSeventh,
                                Interval::MajorSeventh,
                                Interval::MinorNinth,
                                Interval::MajorNinth,
                                Interval::AugmentedNinth,
                                Interval::DiminishedEleventh,
                                Interval::PerfectEleventh,
                                Interval::AugmentedEleventh,
                                Interval::MinorThirteenth,
                                Interval::MajorThirteenth,
                                Interval::AugmentedThirteenth,
                            ]),
                        );
                    }
                    _ => unreachable!(),
                }

                notes.sort();

                // Generate the sample.
                let kord_item = get_simulated_kord_item(noise_asset_root.clone(), &notes, peak_radius, harmonic_decay, frequency_wobble);

                inner_result.push(kord_item);
            }
        }

        inner_result
    });

    results.flatten().collect()
}

/// Get a random item from a list of items.
pub fn get_random_item<T: Copy>(items: &[T]) -> T {
    let mut rng = rand::rng();
    let index = rng.random_range(0..items.len());
    items[index]
}

/// Get a random number between 0 and 1.
pub fn get_random() -> f32 {
    let mut rng = rand::rng();
    rng.random()
}

/// Get a random number between two numbers.
pub fn get_random_between(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

// Tests.

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::ml::base::{helpers::save_kord_item, KordItem, FREQUENCY_SPACE_SIZE};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_kord_item() {
        let destination = Path::new(".hidden/test_data");
        std::fs::create_dir_all(destination).unwrap();

        let item = KordItem {
            path: destination.to_owned(),
            frequency_space: [3f32; FREQUENCY_SPACE_SIZE],
            label: 42,
        };

        let path = save_kord_item(destination, "", "test", &item).unwrap();
        let loaded = load_kord_item(path);

        assert_eq!(item.label, loaded.label);
    }
}
