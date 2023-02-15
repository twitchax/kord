use std::sync::Arc;

use burn::{tensor::backend::{ADBackend, Backend}, optim::{AdamConfig, decay::WeightDecayConfig, Adam}, data::dataloader::DataLoaderBuilder, train::{LearnerBuilder, metric::LossMetric}, config::Config};

use crate::core::note::{Note, HasNoteId};

use super::{data::{KordDataset, KordBatcher, binary_to_u128}, base::{TrainConfig}, model::KordModel, helpers::KordAccuracyMetric};

pub fn run<B: ADBackend>(device: B::Device, config: &TrainConfig) {
    // Define the Adam config.

    let adam_config = AdamConfig::new(config.adam_learning_rate)
        .with_weight_decay(Some(WeightDecayConfig::new(config.adam_weight_decay)))
        .with_beta_1(config.adam_beta1)
        .with_beta_2(config.adam_beta2)
        .with_epsilon(config.adam_epsilon);

    // Define the datasets.

    let (train_dataset, test_dataset) = KordDataset::from_folder(&config.source, config.model_seed);

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

    let learner = LearnerBuilder::new(&config.destination)
        .metric_train_plot(KordAccuracyMetric::new())
        .metric_valid_plot(KordAccuracyMetric::new())
        .metric_train_plot(LossMetric::new())
        .metric_valid_plot(LossMetric::new())
        //.with_file_checkpointer::<f32>(2)
        .devices(vec![device.clone()])
        .num_epochs(config.model_epochs)
        .build(model, optimizer);

    // Train the model.

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    // Save the config.

    config
        .save(format!("{}/config.json", &config.destination).as_str())
        .unwrap();

    // Compute overall accuracy.

    let accuracy = compute_overall_accuracy(&model_trained, &device);

    println!("Overall accuracy: {}%", accuracy);
}

pub(crate) fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> f32 {
    let dataset = KordDataset::from_folder(".hidden/samples", 0);

    let mut kord_items = dataset.0.items;
    kord_items.extend(dataset.1.items);

    let mut correct = 0;
    
    for kord_item in &kord_items {
        let sample = super::data::kord_item_to_sample_tensor(kord_item).to_device(device).detach();
        let target: Vec<f32> = super::data::kord_item_to_target_tensor::<B>(kord_item).into_data().convert().value;

        let inferred = model_trained.forward(sample).to_data().convert().value.into_iter().map(f32::round).collect::<Vec<_>>();

        if target == inferred {
            correct += 1;
        } else {
            let target_array: [_; 128] = target.try_into().unwrap();
            let mut target_notes = Note::from_id_mask(binary_to_u128(&target_array)).unwrap();
            target_notes.sort();
            let target_notes = target_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            let inferred_array: [_; 128] = inferred.try_into().unwrap();
            let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
            inferred_notes.sort();
            let inferred_notes = inferred_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            println!("{} -> {} (inferred) -> {} (target)", kord_item.path.to_string_lossy(), inferred_notes, target_notes);
        }
    }
    
    100.0 * (correct as f32 / kord_items.len() as f32)
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use burn_autodiff::ADBackendDecorator;
    use burn_ndarray::{NdArrayDevice, NdArrayBackend};

    #[test]
    fn test_train() {
        let device = NdArrayDevice::Cpu;
        
        let config = TrainConfig {
            source: "tests/samples".to_string(),
            destination: ".hidden/tmp/".to_string(),
            mlp_layers: 1,
            mlp_size: 256,
            mlp_dropout: 0.3,
            model_epochs: 1,
            model_batch_size: 1,
            model_workers: 1,
            model_seed: 42,
            adam_learning_rate: 1e-4,
            adam_weight_decay: 5e-5,
            adam_beta1: 0.9,
            adam_beta2: 0.999,
            adam_epsilon: 1e-5,
            sigmoid_strength: 10.0,
        };

        run::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config);
    }
}