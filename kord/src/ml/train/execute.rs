//! Training execution.

use std::sync::Arc;

use burn::{
    config::Config,
    data::dataloader::DataLoaderBuilder,
    lr_scheduler::constant::ConstantLr,
    module::Module,
    optim::{decay::WeightDecayConfig, AdamConfig},
    record::{BinFileRecorder, FullPrecisionSettings, Recorder},
    tensor::backend::{AutodiffBackend, Backend},
    train::{
        metric::{HammingScore, LossMetric},
        LearnerBuilder,
    },
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "ml_target_folded")]
use crate::ml::base::helpers::{binary_to_u16, fold_binary, u128_to_binary};
use crate::{
    core::base::Res,
    ml::base::{
        data::{kord_item_to_sample_tensor, kord_item_to_target_tensor},
        helpers::{binary_to_u128, get_deterministic_guess, logits_to_binary_predictions},
        model::KordModel,
        NUM_CLASSES,
    },
};

#[cfg(feature = "ml_hpt")]
use crate::core::base::Void;

use super::data::{KordBatcher, KordDataset};

use crate::ml::base::TrainConfig;

/// Run the training.
///
/// Given the [`TrainConfig`], this function will run the training and return the overall accuracy on
/// the validation / test set.
pub fn run_training<B: AutodiffBackend>(device: B::Device, config: &TrainConfig, print_accuracy_report: bool, save_model: bool) -> Res<f32>
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
    )?;

    // Define the data loaders.

    let batcher_train = KordBatcher::<B>::new(device.clone());
    let batcher_valid = KordBatcher::<B::InnerBackend>::new(device.clone());

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.model_batch_size)
        .shuffle(config.model_seed)
        .num_workers(config.model_workers)
        .build(Arc::new(train_dataset));

    let dataloader_valid = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.model_batch_size)
        .num_workers(config.model_workers)
        .build(Arc::new(valid_dataset));

    // Define the model.

    let optimizer = adam_config.init();
    let model = KordModel::new(&device, config.mha_heads, config.mha_dropout, config.sigmoid_strength);

    let mut learner_builder = LearnerBuilder::new(&config.log)
        //.with_file_checkpointer::<f32>(2)
        .devices(vec![device.clone()])
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

    let model_trained = learner.fit(dataloader_train, dataloader_valid);

    // Save the model.

    if save_model {
        let config_path = format!("{}/model_config.json", &config.destination);
        let state_path = format!("{}/state.json.bin", &config.destination);
        let _ = std::fs::create_dir_all(&config.destination);
        let _ = std::fs::remove_file(&config_path);
        let _ = std::fs::remove_file(&state_path);

        config.save(&config_path)?;
        BinFileRecorder::<FullPrecisionSettings>::new().record(model_trained.clone().into_record(), state_path.into())?;
    }

    // Compute overall accuracy.

    let accuracy = if print_accuracy_report { compute_overall_accuracy(&model_trained, &device)? } else { 0.0 };

    Ok(accuracy)
}

/// Compute the overall accuracy of the model.
#[coverage(off)]
pub fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> Res<f32> {
    let kord_items = KordDataset::from_folder("kord/samples/captured")?.items;

    let mut deterministic_correct = 0;
    let mut inference_correct = 0;

    for kord_item in &kord_items {
        let sample = kord_item_to_sample_tensor(device, kord_item).to_device(device).detach();
        let target: Vec<f32> = kord_item_to_target_tensor::<B>(device, kord_item).into_data().to_vec().unwrap_or_default();
        let target_array: [_; NUM_CLASSES] = target.clone().try_into().unwrap();

        #[cfg(feature = "ml_target_full")]
        let target_binary = binary_to_u128(&target_array);
        #[cfg(feature = "ml_target_folded")]
        let target_binary = binary_to_u16(&target_array);

        let deterministic = get_deterministic_guess(kord_item);
        #[cfg(feature = "ml_target_folded")]
        let deterministic = binary_to_u16(&fold_binary(&u128_to_binary(deterministic)));

        // Forward pass outputs logits, apply sigmoid, and threshold for inference.
        let logits = model_trained.forward(sample).detach();
        let logits_vec: Vec<f32> = logits.into_data().to_vec().unwrap_or_default();

        // Apply sigmoid and a 0.5 threshold.
        let inferred = logits_to_binary_predictions(&logits_vec);

        if target_binary == deterministic {
            deterministic_correct += 1;
        }

        if target == inferred {
            inference_correct += 1;
        }
    }

    let deterministic_accuracy = 100.0 * (deterministic_correct as f32 / kord_items.len() as f32);
    println!("Deterministic accuracy: {deterministic_accuracy}%");

    let inference_accuracy = 100.0 * (inference_correct as f32 / kord_items.len() as f32);
    println!("Inference accuracy: {inference_accuracy}%");

    Ok(inference_accuracy)
}

/// Run hyper parameter tuning.
///
/// This method sweeps through the hyper parameters and runs training for each combination. The best
/// hyper parameters are then printed at the end.
#[cfg(feature = "ml_hpt")]
#[coverage(off)]
pub fn hyper_parameter_tuning(source: String, destination: String, log: String, backend: String) -> Void {
    use burn::backend::Autodiff;

    let peak_radiuses = [2.0];
    let harmonic_decays = [0.1];
    let frequency_wobbles = [0.4];
    let mha_heads = [16]; // Reduced to 4 heads for better per-head capacity
    let mha_dropouts = [0.3]; // Reduced dropout for better learning
    let epochs = [64];
    let learning_rates = [1e-3];
    let weight_decays = [1e-4];

    let mut count = 1;
    let total =
        peak_radiuses.len() * harmonic_decays.len() * frequency_wobbles.len() * mha_heads.len() * mha_dropouts.len() * mha_heads.len() * epochs.len() * learning_rates.len() * weight_decays.len();

    let mut max_accuracy = 0.0;
    let mut best_config = None;

    for peak_radius in &peak_radiuses {
        for harmonic_decay in &harmonic_decays {
            for frequency_wobble in &frequency_wobbles {
                for mha_head in &mha_heads {
                    for mha_dropout in &mha_dropouts {
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
                                        mha_heads: *mha_head,
                                        mha_dropout: *mha_dropout,
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

                                            run_training::<Autodiff<LibTorch<f32>>>(device, &config, true, false)?
                                        }
                                        #[cfg(feature = "ml_ndarray")]
                                        "ndarray" => {
                                            use burn::backend::{ndarray::NdArrayDevice, NdArray};

                                            let device = NdArrayDevice::Cpu;

                                            run_training::<Autodiff<NdArray<f32>>>(device, &config, true, false)?
                                        }
                                        _ => {
                                            return Err(anyhow::Error::msg(
                                                "Invalid device (must choose either `tch` [requires `ml_tch` feature] or `cpu` [requires `ml_ndarray` feature]).",
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
            mha_heads: 16,
            mha_dropout: 0.3,
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

        run_training::<Autodiff<NdArray<f32>>>(device, &config, false, false).unwrap();
    }
}
