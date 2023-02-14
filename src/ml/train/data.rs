//! Module that defines how data is batched and loaded for training.

use std::path::Path;

use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    tensor::{backend::Backend, Data, Tensor},
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::ml::base::{KordItem, MEL_SPACE_SIZE};

use super::base::{load_kord_item, mel_filter_banks_from};

// Dataset.

/// A dataset of kord samples.
pub(crate) struct KordDataset {
    pub items: Vec<KordItem>,
}

impl KordDataset {
    /// Load the kord dataset from the given folder.
    pub fn from_folder(name: impl AsRef<Path>, seed: u64) -> (Self, Self) {
        // First, get all of the *.bin files in the folder.
        let mut files = std::fs::read_dir(name)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().unwrap() == "bin")
            .collect::<Vec<_>>();

        //Convert the seed.
        let mut seed_bytes = [0u8; 32];
        for (i, byte) in seed.to_be_bytes().iter().enumerate() {
            seed_bytes[i] = *byte;
        }

        // Shuffle the files.
        let mut rng = StdRng::from_seed(seed_bytes);
        files.shuffle(&mut rng);

        // Split the files into train and test.
        let (test_files, train_files) = files.split_at(files.len() / 10);

        // Load the files into memory.
        let test_items = test_files.iter().map(|path| load_kord_item(path)).collect();
        let train_items = train_files.iter().map(|path| load_kord_item(path)).collect();

        // Return the train and test datasets.
        let train = Self { items: train_items };
        let test = Self { items: test_items };

        (train, test)
    }
}

impl Dataset<KordItem> for KordDataset {
    fn get(&self, index: usize) -> Option<KordItem> {
        self.items.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Batcher.

pub(crate) struct KordBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> KordBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct KordBatch<B: Backend> {
    pub(crate) frequency_spaces: Tensor<B, 2>,
    pub(crate) targets: Tensor<B, 2>,
}

impl<B: Backend> Batcher<KordItem, KordBatch<B>> for KordBatcher<B> {
    fn batch(&self, items: Vec<KordItem>) -> KordBatch<B> {
        let frequency_spaces = items
            .iter()
            .map(|item| {
                let frequency_space = item.frequency_space;
                let mel_space = mel_filter_banks_from(&frequency_space);

                // Get the max value.
                let max = mel_space.iter().fold(0f32, |acc, &x| acc.max(x));

                // Normalize the mel space peaks.
                let mut result = [0f32; MEL_SPACE_SIZE];
                (0..MEL_SPACE_SIZE).for_each(|k| {
                    result[k] = mel_space[k] / max;
                });

                result
            })
            .map(Data::<f32, 1>::from)
            .map(|data| Tensor::<B, 1>::from_data(data.convert()))
            .map(|tensor| tensor.reshape([1, MEL_SPACE_SIZE]))
            .collect();

        let targets = items
            .iter()
            .map(|item| {
                let num = item.label;

                let mut binary = [0f32; 128];
                for i in 0..128 {
                    binary[127 - i] = (num >> i & 1) as f32;
                }

                // let mut folded = [0f32; 12];

                // for k in 0..10 {
                //     let slice = &binary[k * 12..(k + 1) * 12];

                //     for i in 0..12 {
                //         folded[i] = (slice[i] as f32).max(folded[i]) as f32;
                //     }
                // }

                Data::<f32, 1>::from(binary)
            })
            .map(|data| Tensor::<B, 1>::from_data(data.convert()))
            .map(|tensor| tensor.reshape([1, 128]))
            .collect();

        let frequency_spaces = Tensor::cat(frequency_spaces, 0).to_device(&self.device).detach();
        let targets = Tensor::cat(targets, 0).to_device(&self.device).detach();

        KordBatch { frequency_spaces, targets }
    }
}
