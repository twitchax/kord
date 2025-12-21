//! Training execution.

use std::{cmp::Ordering, sync::Arc};

use burn::{
    config::Config,
    data::dataloader::DataLoaderBuilder,
    lr_scheduler::constant::ConstantLr,
    module::Module,
    optim::{decay::WeightDecayConfig, AdamConfig},
    record::{BinFileRecorder, Recorder},
    tensor::backend::{AutodiffBackend, Backend},
    train::{
        metric::{HammingScore, LossMetric},
        LearnerBuilder, LearningStrategy,
    },
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    core::base::Res,
    ml::base::{
        data::{kord_item_to_sample_tensor, kord_item_to_target_tensor},
        helpers::{logits_to_predictions, logits_to_probabilities},
        model::KordModel,
        KordItem, StorePrecisionSettings, NOTE_SIGNATURE_SIZE, NUM_CLASSES,
    },
};

#[cfg(feature = "ml_hpt")]
use crate::core::base::Void;

use super::data::{KordBatcher, KordDataset};
use crate::ml::base::TrainConfig;

const EVAL_SAMPLE_LIMIT: usize = 1000;
const CAPTURED_DATASET_PATH: &str = "samples/captured";
const CLASS_BREAKDOWN_TOP_K: usize = 10;
const THRESHOLD_DEFAULT: f32 = 0.5;
const THRESHOLD_MIN: f32 = 0.05;
const THRESHOLD_MAX: f32 = 0.95;

/// Run the training.
///
/// Given the [`TrainConfig`], this function will run the training and return the overall accuracy on
/// the validation / test set.
pub fn run_training<B: AutodiffBackend>(device: B::Device, config: &TrainConfig, should_print_accuracy_report: bool, save_model: bool) -> Res<f32>
where
    B::FloatElem: Serialize + DeserializeOwned,
{
    // Define the Adam config.

    let adam_config = AdamConfig::new()
        .with_weight_decay(Some(WeightDecayConfig::new(config.adam_weight_decay)))
        .with_beta_1(config.adam_beta1)
        .with_beta_2(config.adam_beta2)
        .with_epsilon(config.adam_epsilon);

    // Define the datasets.

    let (train_dataset, valid_dataset) = KordDataset::from_sources(
        &config.noise_asset_root,
        &config.training_sources,
        &config.validation_sources,
        config.simulation_size,
        config.simulation_peak_radius,
        config.simulation_harmonic_decay,
        config.simulation_frequency_wobble,
        config.captured_oversample_factor,
    )?;

    let train_dataset = Arc::new(train_dataset);
    let valid_dataset = Arc::new(valid_dataset);

    // Define the data loaders.

    let batcher_train = KordBatcher::<B>::new(device.clone());
    let batcher_valid = KordBatcher::<B::InnerBackend>::new(device.clone());

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.model_batch_size)
        .shuffle(config.model_seed)
        .num_workers(config.model_workers)
        .build(train_dataset.clone());

    let dataloader_valid = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.model_batch_size)
        .num_workers(config.model_workers)
        .build(valid_dataset.clone());

    // Define the model.

    let optimizer = adam_config.init();
    let model = KordModel::new(&device, config.mha_heads, config.dropout, config.trunk_max_hidden_size, config.sigmoid_strength);

    let mut learner_builder = LearnerBuilder::new(&config.log)
        //.with_file_checkpointer::<f32>(2)
        .learning_strategy(LearningStrategy::SingleDevice(device.clone()))
        .num_epochs(config.model_epochs)
        .summary();

    if !config.no_plots {
        learner_builder = learner_builder
            .metric_train_numeric(HammingScore::new())
            .metric_valid_numeric(HammingScore::new())
            .metric_train_numeric(LossMetric::new())
            .metric_valid_numeric(LossMetric::new());
    }

    // let cosine_lr = CosineAnnealingLrSchedulerConfig::new(config.adam_learning_rate, config.model_epochs)
    //    .init()
    //    .map_err(|s| anyhow::Error::msg(format!("Failed to initialize cosine LR scheduler: {s}")))?;
    let constant_lr = ConstantLr::new(config.adam_learning_rate);
    let learner = learner_builder.build(model, optimizer, constant_lr);

    // Train the model.

    let model_trained = learner.fit(dataloader_train, dataloader_valid).model;

    // Compute overall accuracy and collect thresholds.

    let stat_items = valid_dataset.items.iter().take(EVAL_SAMPLE_LIMIT).cloned().collect::<Vec<_>>();
    let stats = collect_prediction_stats(&model_trained, &device, &stat_items, None)?;

    let captured_stats_result = if should_print_accuracy_report {
        Some(load_captured_items(EVAL_SAMPLE_LIMIT).and_then(|items| {
            if items.is_empty() {
                Ok(None)
            } else {
                let thresholds_override = stats.thresholds.as_deref();
                collect_prediction_stats(&model_trained, &device, items.as_slice(), thresholds_override).map(Some)
            }
        }))
    } else {
        None
    };

    if should_print_accuracy_report {
        println!("Validation dataset metrics ({} samples):", stats.total);
        print_accuracy_report("Validation", &stats, CLASS_BREAKDOWN_TOP_K);

        if let Some(result) = &captured_stats_result {
            println!();
            match result {
                Ok(Some(captured_stats)) => {
                    println!("Captured dataset metrics ({} samples):", captured_stats.total);
                    print_accuracy_report("Captured", captured_stats, CLASS_BREAKDOWN_TOP_K);
                }
                Ok(None) => println!("Captured dataset metrics unavailable: no samples found at {CAPTURED_DATASET_PATH}."),
                Err(err) => println!("Captured dataset metrics unavailable: {err}"),
            }
        }
    }

    if save_model {
        let config_path = format!("{}/model_config.json", &config.destination);
        let state_path = format!("{}/state.json.bin", &config.destination);
        let thresholds_path = format!("{}/thresholds.json", &config.destination);

        let _ = std::fs::create_dir_all(&config.destination);
        let _ = std::fs::remove_file(&config_path);
        let _ = std::fs::remove_file(&state_path);
        let _ = std::fs::remove_file(&thresholds_path);

        config.save(&config_path)?;
        BinFileRecorder::<StorePrecisionSettings>::new().record(model_trained.clone().into_record(), state_path.into())?;

        if let Some(thresholds) = &stats.thresholds {
            std::fs::write(&thresholds_path, serde_json::to_vec(thresholds)?)?;
        }
    }

    Ok(stats.inference_accuracy_percent())
}

/// Compute the overall accuracy of the model.
#[coverage(off)]
pub fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> Res<f32> {
    let kord_items = KordDataset::from_folder("samples/captured")?.items;
    let stats = collect_prediction_stats(model_trained, device, &kord_items, None)?;
    print_accuracy_report("Captured", &stats, CLASS_BREAKDOWN_TOP_K);
    Ok(stats.inference_accuracy_percent())
}

struct PredictionStats {
    inference_correct: usize,
    total: usize,
    macro_averages: Option<MacroAverages>,
    pr_metrics: Option<PrMetrics>,
    sample_f1_average: Option<f32>,
    thresholds: Option<Vec<f32>>,
    class_breakdown: Option<Vec<ClassMetric>>,
}

impl PredictionStats {
    fn inference_accuracy_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * (self.inference_correct as f32 / self.total as f32)
        }
    }
}

struct MacroAverages {
    class_count: usize,
    accuracy: f32,
    precision: f32,
    recall: f32,
    f1: f32,
}

struct PrMetrics {
    class_count: usize,
    macro_auc: f32,
}

#[derive(Clone)]
struct ClassMetric {
    index: usize,
    support: usize,
    predicted_positive: usize,
    precision: f32,
    recall: f32,
    f1: f32,
}

struct ClassCounts {
    true_positive: Vec<usize>,
    false_positive: Vec<usize>,
    false_negative: Vec<usize>,
}

impl ClassCounts {
    fn new(size: usize) -> Self {
        Self {
            true_positive: vec![0; size],
            false_positive: vec![0; size],
            false_negative: vec![0; size],
        }
    }

    fn len(&self) -> usize {
        self.true_positive.len()
    }

    fn is_empty(&self) -> bool {
        self.true_positive.is_empty()
    }

    fn update(&mut self, target: &[f32], inferred: &[f32]) -> SampleCounts {
        let mut sample_counts = SampleCounts::default();

        for (index, (&target_value, &inferred_value)) in target.iter().zip(inferred.iter()).enumerate() {
            let target_on = target_value > 0.5;
            let predicted_on = inferred_value > 0.5;

            if predicted_on && target_on {
                self.true_positive[index] += 1;
                sample_counts.true_positive += 1;
            } else if predicted_on && !target_on {
                self.false_positive[index] += 1;
            } else if !predicted_on && target_on {
                self.false_negative[index] += 1;
            }

            if predicted_on {
                sample_counts.predicted_positive += 1;
            }

            if target_on {
                sample_counts.actual_positive += 1;
            }
        }

        sample_counts
    }
}

#[derive(Default)]
struct SampleCounts {
    true_positive: usize,
    predicted_positive: usize,
    actual_positive: usize,
}

struct SampleObservation {
    target: Vec<f32>,
    probabilities: Vec<f32>,
}

struct PrCurves {
    entries: Vec<Vec<(f32, bool)>>,
}

impl PrCurves {
    fn new(size: usize) -> Self {
        Self { entries: vec![Vec::new(); size] }
    }

    fn is_empty(&self) -> bool {
        self.entries.iter().all(Vec::is_empty)
    }

    fn update(&mut self, target: &[f32], probabilities: &[f32]) {
        for (index, (&target_value, &probability)) in target.iter().zip(probabilities.iter()).enumerate() {
            self.entries[index].push((probability, target_value > 0.5));
        }
    }

    fn compute(&self) -> Option<PrMetrics> {
        if self.entries.is_empty() {
            return None;
        }

        let class_count = self.entries.len();
        if class_count == 0 {
            return None;
        }

        let mut auc_sum = 0.0;

        for class_entries in &self.entries {
            auc_sum += pr_auc_for_class(class_entries);
        }

        Some(PrMetrics {
            class_count,
            macro_auc: auc_sum / class_count as f32,
        })
    }

    fn optimal_thresholds(&self) -> Option<Vec<f32>> {
        if self.entries.is_empty() {
            return None;
        }

        let mut thresholds = Vec::with_capacity(self.entries.len());
        let mut has_data = false;

        for class_entries in &self.entries {
            if class_entries.is_empty() {
                thresholds.push(THRESHOLD_DEFAULT);
                continue;
            }

            has_data = true;

            let positives = class_entries.iter().filter(|(_, is_positive)| *is_positive).count();
            if positives == 0 {
                thresholds.push(THRESHOLD_DEFAULT);
                continue;
            }

            let mut best_threshold = THRESHOLD_DEFAULT;
            let mut best_f1 = 0.0;

            for step in 0..=100 {
                let threshold = step as f32 / 100.0;
                let mut tp = 0.0;
                let mut fp = 0.0;
                let mut fn_ = 0.0;

                for &(probability, is_positive) in class_entries {
                    if probability > threshold {
                        if is_positive {
                            tp += 1.0;
                        } else {
                            fp += 1.0;
                        }
                    } else if is_positive {
                        fn_ += 1.0;
                    }
                }

                let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
                let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
                let f1 = if precision + recall > 0.0 { 2.0 * precision * recall / (precision + recall) } else { 0.0 };

                if f1 > best_f1 {
                    best_f1 = f1;
                    best_threshold = threshold;
                }
            }

            let clamped = best_threshold.clamp(THRESHOLD_MIN, THRESHOLD_MAX);
            thresholds.push(clamped);
        }

        if has_data {
            Some(thresholds)
        } else {
            None
        }
    }
}

fn collect_prediction_stats<B: Backend>(model: &KordModel<B>, device: &B::Device, items: &[KordItem], thresholds_override: Option<&[f32]>) -> Res<PredictionStats> {
    let class_count = NUM_CLASSES;
    let macro_class_count = NOTE_SIGNATURE_SIZE.min(NUM_CLASSES);

    let mut pr_curves = PrCurves::new(class_count);
    let mut samples = Vec::with_capacity(items.len());

    for kord_item in items {
        let sample = kord_item_to_sample_tensor(device, kord_item).to_device(device).detach();
        let target: Vec<f32> = kord_item_to_target_tensor::<B>(device, kord_item).into_data().convert::<f32>().to_vec().unwrap_or_default();

        let logits = model.forward(sample).detach();
        let logits_vec: Vec<f32> = logits.into_data().convert::<f32>().to_vec().unwrap_or_default();
        let probabilities = logits_to_probabilities(&logits_vec);

        pr_curves.update(&target, &probabilities);
        samples.push(SampleObservation { target, probabilities });
    }

    let computed_thresholds = if thresholds_override.is_none() { pr_curves.optimal_thresholds() } else { None };

    let thresholds_for_eval = thresholds_override
        .map(|t| t.to_vec())
        .or_else(|| computed_thresholds.clone())
        .unwrap_or_else(|| vec![THRESHOLD_DEFAULT; class_count]);
    let thresholds_slice = thresholds_for_eval.as_slice();

    let mut class_counts = ClassCounts::new(macro_class_count);
    let mut inference_correct = 0;
    let mut sample_f1_sum = 0.0;
    let mut sample_f1_count = 0usize;

    for sample in &samples {
        let predicted = logits_to_predictions(&sample.probabilities, thresholds_slice);

        if sample.target == predicted {
            inference_correct += 1;
        }

        if macro_class_count > 0 {
            let target_slice = &sample.target[..macro_class_count];
            let inferred_slice = &predicted[..macro_class_count];
            let sample_counts = class_counts.update(target_slice, inferred_slice);
            sample_f1_sum += calculate_sample_f1(sample_counts);
            sample_f1_count += 1;
        }
    }

    let macro_averages = if !class_counts.is_empty() { build_macro_metrics(&class_counts, samples.len()) } else { None };
    let pr_metrics = if !pr_curves.is_empty() { pr_curves.compute() } else { None };
    let sample_f1_average = if sample_f1_count > 0 { Some(sample_f1_sum / sample_f1_count as f32) } else { None };
    let class_breakdown = if !class_counts.is_empty() { Some(build_class_breakdown(&class_counts)) } else { None };

    Ok(PredictionStats {
        inference_correct,
        total: samples.len(),
        macro_averages,
        pr_metrics,
        sample_f1_average,
        thresholds: computed_thresholds,
        class_breakdown,
    })
}

fn build_macro_metrics(counts: &ClassCounts, total_samples: usize) -> Option<MacroAverages> {
    if total_samples == 0 || counts.is_empty() {
        return None;
    }

    let class_count = counts.len();
    let mut accuracy_sum = 0.0;
    let mut precision_sum = 0.0;
    let mut recall_sum = 0.0;
    let mut f1_sum = 0.0;

    for index in 0..class_count {
        let tp_count = counts.true_positive[index];
        let fp_count = counts.false_positive[index];
        let fn_count = counts.false_negative[index];

        let tp = tp_count as f32;
        let fp = fp_count as f32;
        let fn_ = fn_count as f32;
        let tn = total_samples.saturating_sub(tp_count + fp_count + fn_count) as f32;

        let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
        let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
        let f1 = if precision + recall > 0.0 { 2.0 * precision * recall / (precision + recall) } else { 0.0 };

        accuracy_sum += (tp + tn) / total_samples as f32;
        precision_sum += precision;
        recall_sum += recall;
        f1_sum += f1;
    }

    let class_count_f32 = class_count as f32;

    Some(MacroAverages {
        class_count,
        accuracy: accuracy_sum / class_count_f32,
        precision: precision_sum / class_count_f32,
        recall: recall_sum / class_count_f32,
        f1: f1_sum / class_count_f32,
    })
}

fn calculate_sample_f1(sample_counts: SampleCounts) -> f32 {
    if sample_counts.predicted_positive == 0 && sample_counts.actual_positive == 0 {
        1.0
    } else if sample_counts.true_positive == 0 {
        0.0
    } else {
        2.0 * sample_counts.true_positive as f32 / (sample_counts.predicted_positive + sample_counts.actual_positive) as f32
    }
}

fn pr_auc_for_class(entries: &[(f32, bool)]) -> f32 {
    if entries.is_empty() {
        return 0.0;
    }

    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    let positives = sorted.iter().filter(|(_, is_positive)| *is_positive).count();
    if positives == 0 {
        return 0.0;
    }

    let positives_f32 = positives as f32;
    let mut true_positive = 0.0;
    let mut false_positive = 0.0;
    let mut previous_recall = 0.0;
    let mut area = 0.0;

    for (_, is_positive) in sorted {
        if is_positive {
            true_positive += 1.0;
        } else {
            false_positive += 1.0;
        }

        let recall = true_positive / positives_f32;
        let precision = true_positive / (true_positive + false_positive);
        area += precision * (recall - previous_recall);
        previous_recall = recall;
    }

    area
}

fn build_class_breakdown(counts: &ClassCounts) -> Vec<ClassMetric> {
    let mut breakdown = Vec::with_capacity(counts.len());

    for index in 0..counts.len() {
        let tp = counts.true_positive[index] as f32;
        let fp = counts.false_positive[index] as f32;
        let fn_ = counts.false_negative[index] as f32;

        let support = (tp + fn_) as usize;
        let predicted_positive = (tp + fp) as usize;
        let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
        let recall = if tp + fn_ > 0.0 { tp / (tp + fn_) } else { 0.0 };
        let f1 = if precision + recall > 0.0 { 2.0 * precision * recall / (precision + recall) } else { 0.0 };

        breakdown.push(ClassMetric {
            index,
            support,
            predicted_positive,
            precision,
            recall,
            f1,
        });
    }

    breakdown
}

fn print_accuracy_report(dataset_label: &str, stats: &PredictionStats, class_breakdown_top_k: usize) {
    println!("Inference accuracy: {:.2}%", stats.inference_accuracy_percent());

    if let Some(macro_averages) = &stats.macro_averages {
        println!("Macro accuracy ({} classes): {:.2}%", macro_averages.class_count, macro_averages.accuracy * 100.0);
        println!("Macro precision ({} classes): {:.2}%", macro_averages.class_count, macro_averages.precision * 100.0);
        println!("Macro recall ({} classes): {:.2}%", macro_averages.class_count, macro_averages.recall * 100.0);
        println!("Macro F1 ({} classes): {:.2}%", macro_averages.class_count, macro_averages.f1 * 100.0);
    }

    if let Some(pr_metrics) = &stats.pr_metrics {
        println!("Macro PR AUC ({} classes): {:.2}%", pr_metrics.class_count, pr_metrics.macro_auc * 100.0);
    }

    if let Some(sample_f1) = stats.sample_f1_average {
        println!("Sample-wise F1: {:.2}%", sample_f1 * 100.0);
    }

    if class_breakdown_top_k == 0 {
        return;
    }

    if let Some(breakdown) = &stats.class_breakdown {
        println!("{} class insights:", dataset_label);

        let mut unsupported: Vec<_> = breakdown.iter().filter(|metric| metric.support == 0).collect();
        if !unsupported.is_empty() {
            unsupported.sort_by(|a, b| b.predicted_positive.cmp(&a.predicted_positive).then_with(|| a.index.cmp(&b.index)));
            let count = unsupported.len().min(class_breakdown_top_k);
            print!("  Zero-support classes (showing {}):", count);
            for metric in unsupported.into_iter().take(count) {
                print!(" {}", metric.index);
            }
            println!();
        }

        let mut low_precision: Vec<_> = breakdown.iter().filter(|metric| metric.support > 0).cloned().collect();
        if !low_precision.is_empty() {
            low_precision.sort_by(|a, b| a.precision.partial_cmp(&b.precision).unwrap_or(Ordering::Equal).then_with(|| a.index.cmp(&b.index)));
            let limit = class_breakdown_top_k.min(low_precision.len());
            println!("  Lowest precision ({}):", limit);
            for metric in low_precision.into_iter().take(limit) {
                println!(
                    "    class {:>3}: support {:>4}, predicted {:>4}, precision {:>6.2}%, recall {:>6.2}%, f1 {:>6.2}%",
                    metric.index,
                    metric.support,
                    metric.predicted_positive,
                    metric.precision * 100.0,
                    metric.recall * 100.0,
                    metric.f1 * 100.0
                );
            }
        }

        let mut low_recall: Vec<_> = breakdown.iter().filter(|metric| metric.support > 0).cloned().collect();
        if !low_recall.is_empty() {
            low_recall.sort_by(|a, b| a.recall.partial_cmp(&b.recall).unwrap_or(Ordering::Equal).then_with(|| a.index.cmp(&b.index)));
            let limit = class_breakdown_top_k.min(low_recall.len());
            println!("  Lowest recall ({}):", limit);
            for metric in low_recall.into_iter().take(limit) {
                println!(
                    "    class {:>3}: support {:>4}, predicted {:>4}, precision {:>6.2}%, recall {:>6.2}%, f1 {:>6.2}%",
                    metric.index,
                    metric.support,
                    metric.predicted_positive,
                    metric.precision * 100.0,
                    metric.recall * 100.0,
                    metric.f1 * 100.0
                );
            }
        }
    }
}

fn load_captured_items(limit: usize) -> Res<Vec<KordItem>> {
    let dataset = KordDataset::from_folder(CAPTURED_DATASET_PATH)?;
    let items = dataset.items.into_iter().take(limit).collect();
    Ok(items)
}

/// Run hyper parameter tuning.
///
/// This method sweeps through the hyper parameters and runs training for each combination. The best
/// hyper parameters are then printed at the end.
#[cfg(feature = "ml_hpt")]
#[coverage(off)]
pub fn hyper_parameter_tuning(source: String, destination: String, log: String, backend: String) -> Void {
    use burn::backend::Autodiff;
    use kord::ml::base::PrecisionElement;

    let peak_radiuses = [2.0];
    let harmonic_decays = [0.1];
    let frequency_wobbles = [0.4];
    let mha_heads = [16]; // Reduced to 4 heads for better per-head capacity
    let dropouts = [0.3]; // Reduced dropout for better learning
    let epochs = [64];
    let learning_rates = [1e-3];
    let weight_decays = [1e-4];

    let mut count = 1;
    let total = peak_radiuses.len() * harmonic_decays.len() * frequency_wobbles.len() * mha_heads.len() * dropouts.len() * mha_heads.len() * epochs.len() * learning_rates.len() * weight_decays.len();

    let mut max_accuracy = 0.0;
    let mut best_config = None;

    for peak_radius in &peak_radiuses {
        for harmonic_decay in &harmonic_decays {
            for frequency_wobble in &frequency_wobbles {
                for mha_head in &mha_heads {
                    for dropout in &dropouts {
                        for epoch in &epochs {
                            for learning_rate in &learning_rates {
                                for weight_decay in &weight_decays {
                                    let config = TrainConfig {
                                        noise_asset_root: "kord/noise".to_string(),
                                        training_sources: vec![source.clone()],
                                        validation_sources: Vec::new(),
                                        destination: destination.clone(),
                                        log: log.clone(),
                                        simulation_size: 100,
                                        simulation_peak_radius: *peak_radius,
                                        simulation_harmonic_decay: *harmonic_decay,
                                        simulation_frequency_wobble: *frequency_wobble,
                                        captured_oversample_factor: 1,
                                        mha_heads: *mha_head,
                                        dropout: *dropout,
                                        trunk_max_hidden_size: 1024,
                                        model_epochs: *epoch as usize,
                                        model_batch_size: 100,
                                        model_workers: 64,
                                        model_seed: 76980,
                                        adam_learning_rate: *learning_rate,
                                        adam_weight_decay: *weight_decay,
                                        adam_beta1: 0.9,
                                        adam_beta2: 0.999,
                                        adam_epsilon: f32::EPSILON,
                                        sigmoid_strength: 1.0,
                                        no_plots: false,
                                    };

                                    println!("Running training {count}/{total}:\n\n{config}\n");

                                    let accuracy = match backend.as_str() {
                                        #[cfg(feature = "ml_tch")]
                                        "tch" => {
                                            #[cfg(not(target_os = "macos"))]
                                            use burn::backend::libtorch::LibTorchDevice;
                                            use burn::backend::LibTorch;

                                            #[cfg(not(target_os = "macos"))]
                                            let device = LibTorchDevice::Cuda(0);
                                            #[cfg(target_os = "macos")]
                                            let device = TchDevice::Mps;

                                            run_training::<Autodiff<LibTorch<PrecisionElement>>>(device, &config, true, false)?
                                        }
                                        #[cfg(feature = "ml_candle")]
                                        "candle" => {
                                            #[cfg(not(target_os = "macos"))]
                                            use burn::backend::candle::CandleDevice;
                                            use burn::backend::Candle;

                                            #[cfg(not(target_os = "macos"))]
                                            let device = CandleDevice::cuda(0);
                                            #[cfg(target_os = "macos")]
                                            let device = CandleDevice::Cpu;

                                            run_training::<Autodiff<Candle<PrecisionElement>>>(device, &config, true, false)?
                                        }
                                        #[cfg(feature = "ml_ndarray")]
                                        "ndarray" => {
                                            use burn::backend::{ndarray::NdArrayDevice, NdArray};

                                            let device = NdArrayDevice::Cpu;

                                            run_training::<Autodiff<NdArray<PrecisionElement>>>(device, &config, true, false)?
                                        }
                                        _ => {
                                            return Err(anyhow::Error::msg(
                                                "Invalid device (must choose either `tch` [requires `ml_tch` feature], `candle` [requires `ml_candle` feature], or `ndarray` [requires `ml_ndarray` feature]).",
                                            ));
                                        }
                                    };

                                    if accuracy > max_accuracy {
                                        println!("New max accuracy: {accuracy}%");

                                        max_accuracy = accuracy;
                                        best_config = Some(config);
                                    }

                                    println!();

                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(best_config) = best_config {
        println!();
        println!();
        println!();
        println!("Best config: {best_config}");
        println!("Best accuracy: {max_accuracy}%");
    }

    Ok(())
}

// Tests.

#[cfg(test)]
#[cfg(feature = "ml_train")]
mod tests {
    use super::*;
    use crate::ml::base::PrecisionElement;
    use burn::backend::{ndarray::NdArrayDevice, Autodiff, NdArray};

    #[test]
    fn test_train() {
        let device = NdArrayDevice::Cpu;

        let config = TrainConfig {
            noise_asset_root: "noise".to_string(),
            training_sources: vec!["tests/samples".to_string()],
            validation_sources: Vec::new(),
            destination: ".hidden/test_model".to_string(),
            log: ".hidden/test_log".to_string(),
            simulation_size: 1,
            simulation_peak_radius: 1.0,
            simulation_harmonic_decay: 0.5,
            simulation_frequency_wobble: 0.5,
            captured_oversample_factor: 1,
            mha_heads: 16,
            dropout: 0.3,
            trunk_max_hidden_size: 1024,
            model_epochs: 1,
            model_batch_size: 10,
            model_workers: 1,
            model_seed: 42,
            adam_learning_rate: 1e-3,
            adam_weight_decay: 1e-4,
            adam_beta1: 0.9,
            adam_beta2: 0.999,
            adam_epsilon: 1e-5,
            sigmoid_strength: 1.0,
            no_plots: true,
        };

        run_training::<Autodiff<NdArray<PrecisionElement>>>(device, &config, false, false).unwrap();
    }
}
