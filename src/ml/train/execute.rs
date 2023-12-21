//! Training execution.

use std::sync::Arc;

use burn::{
    backend::Autodiff,
    config::Config,
    data::dataloader::DataLoaderBuilder,
    lr_scheduler::constant::ConstantLr,
    module::Module,
    optim::{decay::WeightDecayConfig, AdamConfig},
    record::{BinFileRecorder, FullPrecisionSettings, Recorder},
    tensor::backend::{AutodiffBackend, Backend},
    train::{metric::LossMetric, LearnerBuilder},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    core::base::{Res, Void},
    ml::base::{
        data::{kord_item_to_sample_tensor, kord_item_to_target_tensor},
        helpers::{binary_to_u128, get_deterministic_guess},
        model::KordModel,
        NUM_CLASSES,
    },
};

use super::{
    data::{KordBatcher, KordDataset},
    helpers::KordAccuracyMetric,
};

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
        //.with_learning_rate(config.adam_learning_rate)
        .with_weight_decay(Some(WeightDecayConfig::new(config.adam_weight_decay)))
        .with_beta_1(config.adam_beta1)
        .with_beta_2(config.adam_beta2)
        .with_epsilon(config.adam_epsilon);

    // Define the datasets.

    let (train_dataset, test_dataset) = KordDataset::from_folder_and_simulation(
        &config.source,
        config.simulation_size,
        config.simulation_peak_radius,
        config.simulation_harmonic_decay,
        config.simulation_frequency_wobble,
    );

    // Define the data loaders.

    let batcher_train = KordBatcher::<B>::new(device.clone());
    let batcher_valid = KordBatcher::<B::InnerBackend>::new(device.clone());

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.model_batch_size)
        .shuffle(config.model_seed)
        .num_workers(config.model_workers)
        .build(Arc::new(train_dataset));

    let dataloader_test = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.model_batch_size)
        .num_workers(config.model_workers)
        .build(Arc::new(test_dataset));

    // Define the model.

    let optimizer = adam_config.init();
    let model = KordModel::new(config.mha_heads, config.mha_dropout, config.sigmoid_strength);

    let mut learner_builder = LearnerBuilder::new(&config.log)
        //.with_file_checkpointer::<f32>(2)
        .devices(vec![device.clone()])
        .num_epochs(config.model_epochs);

    if !config.no_plots {
        learner_builder = learner_builder
            .metric_train_numeric(KordAccuracyMetric::new())
            .metric_valid_numeric(KordAccuracyMetric::new())
            .metric_train_numeric(LossMetric::new())
            .metric_valid_numeric(LossMetric::new());
    }

    let learner = learner_builder.build(model, optimizer, ConstantLr::new(config.adam_learning_rate));

    // Train the model.

    let model_trained = learner.fit(dataloader_train, dataloader_test);

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

    let accuracy = if print_accuracy_report { compute_overall_accuracy(&model_trained, &device) } else { 0.0 };

    Ok(accuracy)
}

/// Compute the overall accuracy of the model.
#[coverage(off)]
pub fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> f32 {
    let dataset = KordDataset::from_folder_and_simulation("samples", 0, 0.0, 0.0, 0.0);

    let kord_items = dataset.1.items;

    let mut deterministic_correct = 0;
    let mut inferrence_correct = 0;

    for kord_item in &kord_items {
        let sample = kord_item_to_sample_tensor(kord_item).to_device(device).detach();
        let target: Vec<f32> = kord_item_to_target_tensor::<B>(kord_item).into_data().convert().value;
        let target_array: [_; NUM_CLASSES] = target.clone().try_into().unwrap();
        let target_binary = binary_to_u128(&target_array);

        let deterministic = get_deterministic_guess(kord_item);

        let inference = model_trained.forward(sample).to_data().convert().value.into_iter().collect::<Vec<f32>>();
        let inferred = inference.iter().cloned().map(f32::round).collect::<Vec<_>>();

        if target_binary == deterministic {
            deterministic_correct += 1;
        }

        if target == inferred {
            inferrence_correct += 1;
        }
    }

    let deterministic_accuracy = 100.0 * (deterministic_correct as f32 / kord_items.len() as f32);
    println!("Deterministic accuracy: {}%", deterministic_accuracy);

    let inference_accuracy = 100.0 * (inferrence_correct as f32 / kord_items.len() as f32);
    println!("Inference accuracy: {}%", inference_accuracy);

    inference_accuracy
}

/// Run hyper parameter tuning.
///
///This method sweeps through the hyper parameters and runs training for each combination. The best
/// hyper parameters are then printed at the end.
#[coverage(off)]
pub fn hyper_parameter_tuning(source: String, destination: String, log: String, device: String) -> Void {
    let peak_radiuses = [2.0];
    let harmonic_decays = [0.1];
    let frequency_wobbles = [0.4];
    let mha_heads = [8];
    let mha_dropouts = [0.3];
    let epochs = [64];
    let learning_rates = [1e-5];
    let weight_decays = [5e-4];

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
                                        source: source.clone(),
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

                                    println!("Running training {}/{}:\n\n{}\n", count, total, config);

                                    let accuracy = match device.as_str() {
                                        #[cfg(feature = "ml_gpu")]
                                        "gpu" => {
                                            use burn_tch::{LibTorch, LibTorchDevice};

                                            #[cfg(not(target_os = "macos"))]
                                            let device = LibTorchDevice::Cuda(0);
                                            #[cfg(target_os = "macos")]
                                            let device = TchDevice::Mps;

                                            run_training::<Autodiff<LibTorch<f32>>>(device, &config, true, false)?
                                        }
                                        "cpu" => {
                                            use burn_ndarray::{NdArray, NdArrayDevice};

                                            let device = NdArrayDevice::Cpu;

                                            run_training::<Autodiff<NdArray<f32>>>(device, &config, true, false)?
                                        }
                                        _ => {
                                            return Err(anyhow::Error::msg("Invalid device (must choose either `gpu` [requires `ml_gpu` feature] or `cpu`)."));
                                        }
                                    };

                                    if accuracy > max_accuracy {
                                        println!("New max accuracy: {}%", accuracy);

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
        println!("Best config: {}", best_config);
        println!("Best accuracy: {}%", max_accuracy);
    }

    Ok(())
}

// Tests.

#[cfg(test)]
#[cfg(feature = "ml_train")]
mod tests {
    use super::*;
    use burn::backend::Autodiff;
    use burn_ndarray::{NdArray, NdArrayDevice};

    #[test]
    fn test_train() {
        let device = NdArrayDevice::Cpu;

        let config = TrainConfig {
            source: "tests/samples".to_string(),
            destination: ".hidden/test_model".to_string(),
            log: ".hidden/test_log".to_string(),
            simulation_size: 1,
            simulation_peak_radius: 1.0,
            simulation_harmonic_decay: 0.5,
            simulation_frequency_wobble: 0.5,
            mha_heads: 1,
            mha_dropout: 0.3,
            model_epochs: 1,
            model_batch_size: 10,
            model_workers: 1,
            model_seed: 42,
            adam_learning_rate: 1e-4,
            adam_weight_decay: 5e-5,
            adam_beta1: 0.9,
            adam_beta2: 0.999,
            adam_epsilon: 1e-5,
            sigmoid_strength: 1.0,
            no_plots: true,
        };

        run_training::<Autodiff<NdArray<f32>>>(device, &config, false, false).unwrap();
    }
}
