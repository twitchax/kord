//! Helpers for training models.

use burn::{
    module::{Module, ModuleVisitor, ParamId},
    tensor::{
        backend::{AutodiffBackend, Backend},
        Tensor,
    },
    train::{
        metric::{
            state::{FormatOptions, NumericMetricState},
            Adaptor, LossInput, Metric, MetricEntry, MetricMetadata, Numeric,
        },
        TrainOutput, TrainStep, ValidStep,
    },
};
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    core::{
        interval::Interval,
        note::{HasNoteId, Note, ALL_PITCH_NOTES},
        pitch::HasFrequency,
    },
    ml::base::{helpers::load_kord_item, model::KordModel, KordItem, FREQUENCY_SPACE_SIZE, NUM_CLASSES},
};

use super::data::KordBatch;

// Regularization.

#[derive(Debug, Clone, Default)]
struct L1Visitor {
    sum: f32,
}

impl L1Visitor {
    pub fn new() -> Self {
        Self { sum: 0.0 }
    }

    pub fn sum(&self) -> f32 {
        self.sum
    }
}

impl<B: Backend> ModuleVisitor<B> for L1Visitor {
    fn visit_float<const D: usize>(&mut self, _: &ParamId, tensor: &Tensor<B, D>) {
        let sum: f32 = tensor.clone().powf_scalar(2.0).sum().into_data().as_slice().unwrap_or_default()[0];
        self.sum += sum;
    }
}

/// Compute the L1 regularization penalty.
pub fn l1_regularization<B: Backend>(model: &KordModel<B>, lambda: f32) -> f32 {
    let mut visitor = L1Visitor::new();
    model.visit(&mut visitor);
    let sum = visitor.sum();

    sum * lambda
}

// Classification.

/// The [classification](TrainStep) input type.
pub struct KordClassificationOutput<B: Backend> {
    /// The loss tensor.
    pub loss: Tensor<B, 1>,
    /// The output tensor.
    pub output: Tensor<B, 2>,
    /// The target tensor.
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

impl<B: AutodiffBackend> TrainStep<KordBatch<B>, KordClassificationOutput<B>> for KordModel<B> {
    fn step(&self, item: KordBatch<B>) -> TrainOutput<KordClassificationOutput<B>> {
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

/// The [accuracy metric](Metric) for kord samples.
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

    const NAME: &'static str = "Accuracy";

    fn update(&mut self, input: &KordAccuracyInput<B>, _metadata: &MetricMetadata) -> MetricEntry {
        let [batch_size, _n_classes] = input.targets.dims();
        let device = B::Device::default();

        let targets = input.targets.clone().to_device(&device);
        let outputs = input.outputs.clone().to_device(&device);

        let target_round = targets.greater_equal_elem(0.5).int();
        let output_round = outputs.greater_equal_elem(0.5).int();

        let counts: Vec<i64> = target_round.equal(output_round).int().sum_dim(1).into_data().to_vec().unwrap();

        let accuracy = 100.0 * counts.iter().filter(|&&x| x == NUM_CLASSES as i64).count() as f64 / counts.len() as f64;

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

/// Create a simulated kord sample item from a noise basis and a semi-random collection of notes.
pub fn get_simulated_kord_item(notes: &[Note], peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> KordItem {
    let wobble_divisor = 35.0;

    let mut result = match get_random_between(0.0, 4.0).round() as u32 {
        0 | 4 => load_kord_item("assets/no_noise.bin"),
        1 => load_kord_item("assets/pink_noise.bin"),
        2 => load_kord_item("assets/white_noise.bin"),
        3 => load_kord_item("assets/brown_noise.bin"),
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
pub fn get_simulated_kord_items(count: usize, peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> Vec<KordItem> {
    let results = (0..count).into_par_iter().map(|_| {
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
                let kord_item = get_simulated_kord_item(&notes, peak_radius, harmonic_decay, frequency_wobble);

                inner_result.push(kord_item);
            }
        }

        inner_result
    });

    results.flatten().collect()
}

/// Get a random item from a list of items.
pub fn get_random_item<T: Copy>(items: &[T]) -> T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..items.len());
    items[index]
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
