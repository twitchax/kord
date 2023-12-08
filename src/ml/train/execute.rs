use std::sync::Arc;

use burn::{
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::Module,
    optim::{decay::WeightDecayConfig, Adam, AdamConfig},
    tensor::backend::{ADBackend, Backend},
    train::{metric::LossMetric, LearnerBuilder},
};
use burn_autodiff::ADBackendDecorator;
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

pub fn run_training<B: ADBackend>(device: B::Device, config: &TrainConfig, print_accuracy_report: bool, save_model: bool) -> Res<f32>
where
    B::FloatElem: Serialize + DeserializeOwned,
{
    // Define the Adam config.

    let adam_config = AdamConfig::new(config.adam_learning_rate)
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

    let batcher_train = Arc::new(KordBatcher::<B>::new(device.clone()));
    let batcher_valid = Arc::new(KordBatcher::<B::InnerBackend>::new(device.clone()));

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

    let optimizer = Adam::new(&adam_config);
    let model = KordModel::new(config.mlp_layers, config.mlp_size, config.mlp_dropout, config.sigmoid_strength);

    let mut learner_builder = LearnerBuilder::new(&config.log)
        //.with_file_checkpointer::<f32>(2)
        .devices(vec![device.clone()])
        .num_epochs(config.model_epochs);

    if !config.no_plots {
        learner_builder = learner_builder
            .metric_train_plot(KordAccuracyMetric::new())
            .metric_valid_plot(KordAccuracyMetric::new())
            .metric_train_plot(LossMetric::new())
            .metric_valid_plot(LossMetric::new());
    }

    let learner = learner_builder.build(model, optimizer);

    // Train the model.

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    // Save the model.

    if save_model {
        let config_path = format!("{}/model_config.json", &config.destination);
        let state_path = format!("{}/state.json.gz", &config.destination);
        let state_bincode_path = format!("{}/state.bincode", &config.destination);
        let _ = std::fs::create_dir_all(&config.destination);
        let _ = std::fs::remove_file(&config_path);
        let _ = std::fs::remove_file(&state_path);
        let _ = std::fs::remove_file(&state_bincode_path);

        config.save(&config_path)?;
        model_trained.state().save(&state_path)?;
        std::fs::write(&state_bincode_path, bincode::serde::encode_to_vec(&model_trained.state(), bincode::config::standard())?)?;
    }

    // Compute overall accuracy.

    let accuracy = if print_accuracy_report { compute_overall_accuracy(&model_trained, &device) } else { 0.0 };

    Ok(accuracy)
}

#[coverage(off)]
pub fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> f32 {
    let dataset = KordDataset::from_folder_and_simulation("samples", 0, 0.0, 0.0, 0.0);

    let kord_items = dataset.1.items;
    //kord_items.extend(dataset.0.items);

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

        //let strength = inference.iter().zip(inferred.iter()).map(|(l, r)| l * r).filter(|v| *v > 0.1).collect::<Vec<_>>();

        if target_binary == deterministic {
            deterministic_correct += 1;
        } else {
            // let mut target_notes = Note::from_id_mask(target_binary).unwrap();
            // target_notes.sort();
            // let target_notes = target_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            // let mut deterministic_notes = Note::from_id_mask(deterministic).unwrap();
            // deterministic_notes.sort();
            // let deterministic_notes = deterministic_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            // println!();
            // println!(
            //     "{:>60} -> {:>20} (determninistic) -> {:>20} (target)",
            //     kord_item.path.to_string_lossy(),
            //     deterministic_notes,
            //     target_notes
            // );
        }

        if target == inferred {
            inferrence_correct += 1;
        } else {
            // let mut target_notes = Note::from_id_mask(target_binary).unwrap();
            // target_notes.sort();
            // let target_notes = target_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            // let mut deterministic_notes = Note::from_id_mask(deterministic).unwrap();
            // deterministic_notes.sort();
            // let deterministic_notes = deterministic_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            // let inferred_array: [_; 128] = inferred.clone().try_into().unwrap();
            // let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
            // inferred_notes.sort();
            // let inferred_notes = inferred_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            // println!();
            // println!("{:?}", strength);
            // println!(
            //     "{:>60} -> {:>20} (inferred) -> {:>20} (target)",
            //     kord_item.path.to_string_lossy(),
            //     inferred_notes,
            //     target_notes
            // );
        }
    }

    let deterministic_accuracy = 100.0 * (deterministic_correct as f32 / kord_items.len() as f32);
    println!("Deterministic accuracy: {}%", deterministic_accuracy);

    let inference_accuracy = 100.0 * (inferrence_correct as f32 / kord_items.len() as f32);
    println!("Inference accuracy: {}%", inference_accuracy);

    inference_accuracy
}

#[coverage(off)]
pub fn hyper_parameter_tuning(source: String, destination: String, log: String, device: String) -> Void {
    let peak_radiuses = [1.0];
    let harmonic_decays = [0.1];
    let frequency_wobbles = [0.4];
    let mlp_layers = [3];
    let mlp_sizes = [4096];
    let mlp_dropouts = [0.1, 0.3, 0.5];
    let epochs = [256];
    let learning_rates = [1e-5];
    let weight_decays = [5e-4];

    let mut count = 1;
    let total =
        peak_radiuses.len() * harmonic_decays.len() * frequency_wobbles.len() * mlp_layers.len() * mlp_dropouts.len() * mlp_sizes.len() * epochs.len() * learning_rates.len() * weight_decays.len();

    let mut max_accuracy = 0.0;
    let mut best_config = None;

    for peak_radius in &peak_radiuses {
        for harmonic_decay in &harmonic_decays {
            for frequency_wobble in &frequency_wobbles {
                for mlp_layer in &mlp_layers {
                    for mlp_size in &mlp_sizes {
                        for mlp_dropout in &mlp_dropouts {
                            for epoch in &epochs {
                                for learning_rate in &learning_rates {
                                    for weight_decay in &weight_decays {
                                        let config = TrainConfig {
                                            source: source.clone(),
                                            destination: destination.clone(),
                                            log: log.clone(),
                                            simulation_size: 20,
                                            simulation_peak_radius: *peak_radius,
                                            simulation_harmonic_decay: *harmonic_decay,
                                            simulation_frequency_wobble: *frequency_wobble,
                                            mlp_layers: *mlp_layer,
                                            mlp_size: *mlp_size,
                                            mlp_dropout: *mlp_dropout,
                                            model_epochs: *epoch as usize,
                                            model_batch_size: 100,
                                            model_workers: 32,
                                            model_seed: 76980,
                                            adam_learning_rate: *learning_rate,
                                            adam_weight_decay: *weight_decay,
                                            adam_beta1: 0.9,
                                            adam_beta2: 0.999,
                                            adam_epsilon: f32::EPSILON,
                                            sigmoid_strength: 1.0,
                                            no_plots: true,
                                        };

                                        println!("Running training {}/{}:\n\n{}\n", count, total, config);

                                        let accuracy = match device.as_str() {
                                            #[cfg(feature = "ml_gpu")]
                                            "gpu" => {
                                                use burn_tch::{TchBackend, TchDevice};

                                                #[cfg(not(target_os = "macos"))]
                                                let device = TchDevice::Cuda(0);
                                                #[cfg(target_os = "macos")]
                                                let device = TchDevice::Mps;

                                                run_training::<ADBackendDecorator<TchBackend<f32>>>(device, &config, true, false)?
                                            }
                                            "cpu" => {
                                                use burn_ndarray::{NdArrayBackend, NdArrayDevice};

                                                let device = NdArrayDevice::Cpu;

                                                run_training::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config, true, false)?
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
    use burn_autodiff::ADBackendDecorator;
    use burn_ndarray::{NdArrayBackend, NdArrayDevice};

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
            mlp_layers: 1,
            mlp_size: 64,
            mlp_dropout: 0.3,
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

        run_training::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config, false, false).unwrap();
    }
}
