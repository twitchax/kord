use std::sync::Arc;

use burn::{
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::{Module},
    optim::{decay::WeightDecayConfig, Adam, AdamConfig},
    tensor::backend::{ADBackend, Backend},
    train::{metric::LossMetric, LearnerBuilder},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    core::{
        base::{Void},
    },
    ml::base::{data::{kord_item_to_sample_tensor, kord_item_to_target_tensor}, model::KordModel, helpers::{binary_to_u128, get_deterministic_guess}},
};

use super::{
    helpers::KordAccuracyMetric, data::{KordDataset, KordBatcher},
};

use crate::ml::base::TrainConfig;

pub fn run_training<B: ADBackend>(device: B::Device, config: &TrainConfig, print_accuracy_report: bool) -> Void
where
    B::Elem: Serialize + DeserializeOwned,
{
    // Define the Adam config.

    let adam_config = AdamConfig::new(config.adam_learning_rate)
        .with_weight_decay(Some(WeightDecayConfig::new(config.adam_weight_decay)))
        .with_beta_1(config.adam_beta1)
        .with_beta_2(config.adam_beta2)
        .with_epsilon(config.adam_epsilon);

    // Define the datasets.

    let (train_dataset, test_dataset) = KordDataset::from_folder_and_simulation(&config.source, config.simulation_size);

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

    let learner = LearnerBuilder::new(&config.log)
        .metric_train_plot(KordAccuracyMetric::new())
        .metric_valid_plot(KordAccuracyMetric::new())
        .metric_train_plot(LossMetric::new())
        .metric_valid_plot(LossMetric::new())
        //.grads_accumulation(2)
        //.with_file_checkpointer::<f32>(2)
        .devices(vec![device.clone()])
        .num_epochs(config.model_epochs)
        .build(model, optimizer);

    // Train the model.

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    // Save the model.

    let config_path = format!("{}/model_config.json", &config.destination);
    let state_path = format!("{}/state.json.gz", &config.destination);
    let _ = std::fs::create_dir_all(&config.destination);
    let _ = std::fs::remove_file(&config_path);
    let _ = std::fs::remove_file(&state_path);

    config.save(&config_path)?;
    model_trained.state().save(&state_path)?;

    // Compute overall accuracy.

    if print_accuracy_report {
        compute_overall_accuracy(&model_trained, &device);
    }

    Ok(())
}

#[no_coverage]
pub fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) {
    let dataset = KordDataset::from_folder_and_simulation(".hidden/samples", 0);

    let kord_items = dataset.1.items;
    //kord_items.extend(dataset.1.items);

    let mut deterministic_correct = 0;
    let mut inferrence_correct = 0;

    for kord_item in &kord_items {
        let sample = kord_item_to_sample_tensor(kord_item).to_device(device).detach();
        let target: Vec<f32> = kord_item_to_target_tensor::<B>(kord_item).into_data().convert().value;
        let target_array: [_; 128] = target.clone().try_into().unwrap();
        let target_binary = binary_to_u128(&target_array);

        let deterministic = get_deterministic_guess(kord_item);

        let inferred = model_trained.forward(sample).to_data().convert().value.into_iter().map(f32::round).collect::<Vec<_>>();

        if target_binary == deterministic {
            deterministic_correct += 1;
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

            // println!(
            //     "{:>60} -> {:>20} (deterministic) -> {:>20} (inferred) -> {:>20} (target)",
            //     kord_item.path.to_string_lossy(),
            //     deterministic_notes,
            //     inferred_notes,
            //     target_notes
            // );
        }
    }

    let deterministic_accuracy = 100.0 * (deterministic_correct as f32 / kord_items.len() as f32);
    println!("Deterministic accuracy: {}%", deterministic_accuracy);

    let inference_accuracy = 100.0 * (inferrence_correct as f32 / kord_items.len() as f32);
    println!("Inference accuracy: {}%", inference_accuracy);
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
        };

        run_training::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config, false).unwrap();
    }
}
