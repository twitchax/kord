use std::sync::Arc;

use burn::{
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::{Module, State},
    optim::{decay::WeightDecayConfig, Adam, AdamConfig},
    tensor::backend::{ADBackend, Backend},
    train::{metric::LossMetric, LearnerBuilder},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    core::{
        base::{Res, Void},
        note::{HasNoteId, Note},
    },
    ml::base::KordItem,
};

use super::{
    base::TrainConfig,
    data::{binary_to_u128, get_deterministic_guess, KordBatcher, KordDataset},
    helpers::KordAccuracyMetric,
    model::KordModel,
};

#[cfg(feature = "ml_train")]
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
        let accuracy = compute_overall_accuracy(&model_trained, &device);
        println!("Overall accuracy: {}%", accuracy);
    }

    Ok(())
}

#[cfg(feature = "ml_infer")]
pub fn run_inference<B: Backend>(device: &B::Device, kord_item: &KordItem) -> Res<Vec<Note>>
where
    B::Elem: Serialize + DeserializeOwned,
{
    // Load the config and state.

    // [TODO] Read this from within the binary itself.
    let config = TrainConfig::load("model/model_config.json")?;
    let state = State::<B::Elem>::load("model/state.json.gz")?;

    // Define the model.
    let mut model = KordModel::<B>::new(config.mlp_layers, config.mlp_size, config.mlp_dropout, config.sigmoid_strength);
    model.load(&state)?;

    // Prepare the sample.
    let sample = super::data::kord_item_to_sample_tensor(kord_item).to_device(device).detach();

    // Run the inference.
    let inferred = model.forward(sample).to_data().convert().value.into_iter().map(f32::round).collect::<Vec<_>>();
    let inferred_array: [_; 128] = inferred.try_into().unwrap();
    let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
    inferred_notes.sort();

    Ok(inferred_notes)
}

#[no_coverage]
pub(crate) fn compute_overall_accuracy<B: Backend>(model_trained: &KordModel<B>, device: &B::Device) -> f32 {
    let dataset = KordDataset::from_folder(".hidden/samples", 0);

    let mut kord_items = dataset.0.items;
    kord_items.extend(dataset.1.items);

    let mut correct = 0;

    for kord_item in &kord_items {
        let sample = super::data::kord_item_to_sample_tensor(kord_item).to_device(device).detach();
        let target: Vec<f32> = super::data::kord_item_to_target_tensor::<B>(kord_item).into_data().convert().value;

        let deterministic = get_deterministic_guess(kord_item);

        let inferred = model_trained.forward(sample).to_data().convert().value.into_iter().map(f32::round).collect::<Vec<_>>();

        if target == inferred {
            correct += 1;
        } else {
            let target_array: [_; 128] = target.try_into().unwrap();
            let mut target_notes = Note::from_id_mask(binary_to_u128(&target_array)).unwrap();
            target_notes.sort();
            let target_notes = target_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            let mut deterministic_notes = Note::from_id_mask(deterministic).unwrap();
            deterministic_notes.sort();
            let deterministic_notes = deterministic_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            let inferred_array: [_; 128] = inferred.try_into().unwrap();
            let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
            inferred_notes.sort();
            let inferred_notes = inferred_notes.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

            println!(
                "{:>60} -> {:>20} (deterministic) -> {:>20} (inferred) -> {:>20} (target)",
                kord_item.path.to_string_lossy(),
                deterministic_notes,
                inferred_notes,
                target_notes
            );
        }
    }

    100.0 * (correct as f32 / kord_items.len() as f32)
}

// Tests.

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::{
        analyze::base::{get_frequency_space, get_smoothed_frequency_space},
        ml::base::FREQUENCY_SPACE_SIZE,
    };

    use super::*;
    use burn_autodiff::ADBackendDecorator;
    use burn_ndarray::{NdArrayBackend, NdArrayDevice};

    #[test]
    #[cfg(feature = "ml_train")]
    fn test_train() {
        let device = NdArrayDevice::Cpu;

        let config = TrainConfig {
            source: "tests/samples".to_string(),
            destination: ".hidden/test_model".to_string(),
            log: ".hidden/test_log".to_string(),
            mlp_layers: 1,
            mlp_size: 64,
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

        run_training::<ADBackendDecorator<NdArrayBackend<f32>>>(device, &config, false).unwrap();
    }

    #[test]
    #[cfg(feature = "ml_infer")]
    fn test_inference() {
        use crate::core::{base::Parsable, chord::Chord};

        let mut file = File::open("tests/vec.bin").unwrap();
        let file_size = file.metadata().unwrap().len() as usize;
        let float_size = std::mem::size_of::<f32>();
        let element_count = file_size / float_size;
        let mut buffer = vec![0u8; file_size];

        // Read the contents of the file into the buffer
        file.read_exact(&mut buffer).unwrap();

        // Convert the buffer to a vector of f32
        let audio_data: Vec<f32> = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const f32, element_count).to_vec() };

        // Prepare the audio data.
        let frequency_space = get_frequency_space(&audio_data, 5);
        let smoothed_frequency_space: [_; FREQUENCY_SPACE_SIZE] = get_smoothed_frequency_space(&frequency_space, 5)
            .into_iter()
            .take(FREQUENCY_SPACE_SIZE)
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let kord_item = KordItem {
            frequency_space: smoothed_frequency_space,
            ..Default::default()
        };

        let device = NdArrayDevice::Cpu;

        // Run the inference.
        let notes = super::run_inference::<NdArrayBackend<f32>>(&device, &kord_item).unwrap();

        let chord = Chord::from_notes(&notes).unwrap();

        assert_eq!(chord[0], Chord::parse("C7b9").unwrap());
    }
}
