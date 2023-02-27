//! Helpers for training models.

use burn::{
    tensor::{backend::{Backend, ADBackend}, Tensor},
    train::{metric::{
        state::{FormatOptions, NumericMetricState},
        Adaptor, LossInput, Metric, MetricEntry, Numeric,
    }, TrainStep, TrainOutput, ValidStep},
};
use rand::Rng;

use crate::{
    core::{note::{Note, HasNoteId, ALL_PITCH_NOTES}, pitch::HasFrequency, interval::{Interval, PRIMARY_HARMONIC_SERIES}},
    ml::base::{KordItem, FREQUENCY_SPACE_SIZE, NUM_CLASSES, model::KordModel, helpers::load_kord_item},
};

use super::data::KordBatch;

// Loss function.

#[derive(Debug, Clone, Default)]
pub struct MeanSquareLoss<B: Backend> {
    _b: B,
}

impl<B: Backend> MeanSquareLoss<B> {
    pub fn forward(&self, outputs: &Tensor<B, 2>, targets: &Tensor<B, 2>) -> Tensor<B, 1> {
        // Compute the mean square error loss.
        outputs.sub(targets).powf(2.0).mean()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BinaryCrossEntropyLoss<B: Backend> {
    _b: B,
}

impl<B: Backend> BinaryCrossEntropyLoss<B> {
    pub fn forward(&self, outputs: &Tensor<B, 2>, targets: &Tensor<B, 2>) -> Tensor<B, 1> {
        let outputs = outputs.mul_scalar(0.999999f32);
        let result = (targets.mul(&outputs.log()) + (targets.neg().add_scalar(1.0f32)).mul(&(outputs.neg().add_scalar(1.0f32)).log()))
            .mean()
            .neg();

        let value: f32 = result.to_data().convert().value[0];

        if value.is_nan() {
            panic!("NaN loss");
        }

        result
    }
}

// Classification.

pub struct KordClassificationOutput<B: Backend> {
    pub loss: Tensor<B, 1>,
    pub output: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
}

impl<B: Backend> Adaptor<KordAccuracyInput<B>> for KordClassificationOutput<B> {
    fn adapt(&self) -> KordAccuracyInput<B> {
        KordAccuracyInput {
            outputs: self.output.clone(),
            targets: self.targets.clone(),
        }
    }
}

impl<B: Backend> Adaptor<LossInput<B>> for KordClassificationOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}

// Classification adapters.

impl<B: ADBackend> TrainStep<B, KordBatch<B>, KordClassificationOutput<B>> for KordModel<B> {
    fn step(&self, item: KordBatch<B>) -> TrainOutput<B, KordClassificationOutput<B>> {
        let item = self.forward_classification(item);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<KordBatch<B>, KordClassificationOutput<B>> for KordModel<B> {
    fn step(&self, item: KordBatch<B>) -> KordClassificationOutput<B> {
        self.forward_classification(item)
    }
}

// Accuracy metrics.

#[derive(Default)]
pub struct KordAccuracyMetric<B: Backend> {
    state: NumericMetricState,
    _b: B,
}

/// The [accuracy metric](AccuracyMetric) input type.
pub struct KordAccuracyInput<B: Backend> {
    outputs: Tensor<B, 2>,
    targets: Tensor<B, 2>,
}

impl<B: Backend> KordAccuracyMetric<B> {
    /// Create the metric.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<B: Backend> Metric for KordAccuracyMetric<B> {
    type Input = KordAccuracyInput<B>;

    fn update(&mut self, input: &KordAccuracyInput<B>) -> MetricEntry {
        let [batch_size, _n_classes] = input.targets.dims();
        let device = B::Device::default();

        let targets = input.targets.to_device(&device);
        let outputs = input.outputs.to_device(&device);

        // let mse: f64 = targets.sub(&outputs).powf(2.0).mean().to_data().convert().value[0];
        // let rmse = mse.sqrt();

        // let accuracy = 100.0 * (1.0 - rmse);

        let target_round = targets.greater_equal_scalar(0.5).to_int();
        let output_round = outputs.greater_equal_scalar(0.5).to_int();

        let counts: Vec<u8> = target_round.equal(&output_round).to_int().sum_dim(1).into_data().convert().value;

        let accuracy = 100.0 * counts.iter().filter(|&&x| x == NUM_CLASSES as u8).count() as f64 / counts.len() as f64;

        // let loss: f64 = (targets.mul(&outputs.log()) + (targets.neg().add_scalar(1.0)).mul(&outputs.neg().add_scalar(1.0).log())).mean().neg().to_data().convert().value[0];
        // let accuracy = 100.0 * (1.0 - loss);

        self.state.update(accuracy, batch_size, FormatOptions::new("Accuracy").unit("%").precision(2))
    }

    fn clear(&mut self) {
        self.state.reset()
    }
}

impl<B: Backend> Numeric for KordAccuracyMetric<B> {
    fn value(&self) -> f64 {
        self.state.value()
    }
}

// Operations for simulating kord samples.

pub fn get_simulated_kord_item(notes: &[Note], peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> KordItem {
    let mut result = match get_random_between(0.0, 4.0).round() as u32 {
        0 | 4 => load_kord_item("assets/no_noise.bin"),
        1 => load_kord_item("assets/pink_noise.bin"),
        2 => load_kord_item("assets/white_noise.bin"),
        3 => load_kord_item("assets/brown_noise.bin"),
        _ => unreachable!(),
    };

    for note in notes {
        let mut harmonic_strength = 1.0;
        let note_frequency = note.frequency() + get_random_between(-frequency_wobble, frequency_wobble);

        let true_harmonic_series = (1..31).into_iter().map(|k| k as f32 * note_frequency).collect::<Vec<_>>();

        let mut equal_temperament_harmonic_series = PRIMARY_HARMONIC_SERIES.into_iter().map(|k| (*note + k).frequency() + get_random_between(-frequency_wobble, frequency_wobble)).collect::<Vec<_>>();
        equal_temperament_harmonic_series.insert(0, note_frequency);

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

pub fn get_simulated_kord_items(count: usize) -> Vec<KordItem> {
    let mut result = Vec::with_capacity(2960);

    for _ in 0..count {
        for note in ALL_PITCH_NOTES.iter().skip(24).take(60) {
            let note = *note;

            for k in 0..4 {
                let mut notes = vec![note];

                match k {
                    0 => {}
                    1 => {
                        notes.push(note + Interval::MajorThird);
                    }
                    2 => {
                        notes.push(note + Interval::MajorThird);
                        notes.push(note + Interval::PerfectFifth);
                    }
                    3 => {
                        notes.push(note + Interval::MajorThird);
                        notes.push(note + Interval::PerfectFifth);
                        notes.push(note + Interval::MajorSeventh);
                    }
                    _ => unreachable!(),
                }

                notes.sort();

                // Generate the sample.
                let kord_item = get_simulated_kord_item(&notes, 1.0, 0.5, 1.0);

                result.push(kord_item);
            }
        }
    }

    result
}

/// Get a random number between 0 and 1.
pub fn get_random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Get a random number between two numbers.
pub fn get_random_between(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// Tests.

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::ml::base::{KordItem, FREQUENCY_SPACE_SIZE, helpers::save_kord_item};

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
