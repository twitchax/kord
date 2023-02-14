use std::sync::Arc;

use burn::{tensor::backend::ADBackend, optim::{AdamConfig, decay::WeightDecayConfig, Adam}, data::dataloader::DataLoaderBuilder, train::{LearnerBuilder, metric::LossMetric}, config::Config};

use super::{data::{KordDataset, KordBatcher}, base::TrainConfig, model::KordModel, helpers::KordAccuracyMetric};

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
        .devices(vec![device])
        .num_epochs(config.model_epochs)
        .build(model, optimizer);

    // Train the model.

    let _model_trained = learner.fit(dataloader_train, dataloader_test);

    // Save the config.

    config
        .save(format!("{}/config.json", &config.destination).as_str())
        .unwrap();
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