//! Model definition for the Kord model.

use burn::{
    module::{Module, Param},
    nn::{self},
    tensor::{
        backend::{ADBackend, Backend},
        Tensor,
    },
    train::{TrainOutput, TrainStep, ValidStep},
};

use crate::ml::base::{NUM_CLASSES, MEL_SPACE_SIZE};

use super::{
    data::KordBatch,
    helpers::{KordClassificationOutput, MeanSquareLoss, Sigmoid},
    mlp::Mlp,
};

#[derive(Module, Debug)]
pub(crate) struct KordModel<B: Backend> {
    mlp: Param<Mlp<B>>,
    input: Param<nn::Linear<B>>,
    output: Param<nn::Linear<B>>,
    sigmoid: Sigmoid,
}

impl<B: Backend> KordModel<B> {
    pub(crate) fn new(mlp_layers: usize, mlp_size: usize, mlp_dropout: f64, sigmoid_strength: f32) -> Self {
        let input = nn::Linear::new(&nn::LinearConfig::new(MEL_SPACE_SIZE, mlp_size));
        let mlp = Mlp::new(mlp_layers, mlp_size, mlp_dropout);
        let output = nn::Linear::new(&nn::LinearConfig::new(mlp_size, NUM_CLASSES));
        let sigmoid = Sigmoid::new(sigmoid_strength);

        Self {
            mlp: Param::new(mlp),
            output: Param::new(output),
            input: Param::new(input),
            sigmoid,
        }
    }

    pub(crate) fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;

        x = self.input.forward(x);

        //x = self.mlp.forward(x);

        x = self.output.forward(x);

        x = self.sigmoid.forward(x);

        x
    }

    pub(crate) fn forward_classification(&self, item: KordBatch<B>) -> KordClassificationOutput<B> {
        let targets = item.targets;
        let output = self.forward(item.frequency_spaces);

        let loss = MeanSquareLoss::new();
        let loss = loss.forward(&output, &targets);

        KordClassificationOutput { loss, output, targets }
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
