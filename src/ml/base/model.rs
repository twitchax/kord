//! Model definition for the Kord model.

use core::f32;

use burn::{
    module::{Module, Param},
    nn::{self, attention::{MultiHeadAttentionConfig, MultiHeadAttention, MhaInput}, gru::{GruConfig, Gru}, pool::AvgPool1d, EmbeddingConfig, LayerNorm},
    tensor::{backend::Backend, Tensor},
};

use super::{helpers::Sigmoid, mlp::Mlp, INPUT_SPACE_SIZE, NUM_CLASSES};

#[cfg(feature = "ml_train")]
use crate::ml::train::{
    data::KordBatch,
    helpers::{KordClassificationOutput, MeanSquareLoss},
};

#[derive(Module, Debug)]
pub struct KordModel<B: Backend> {
    mha: MultiHeadAttention<B>,
    input: nn::Linear<B>,
    mlp: Mlp<B>,
    output: nn::Linear<B>,
    sigmoid: Sigmoid<B>,
}

impl<B: Backend> KordModel<B> {
    pub fn new(mlp_layers: usize, mlp_size: usize, mlp_dropout: f64, sigmoid_strength: f32) -> Self {
        let mha = MultiHeadAttentionConfig::new(INPUT_SPACE_SIZE, 128).init::<B>();
        let input = nn::LinearConfig::new(INPUT_SPACE_SIZE, mlp_size).init::<B>();
        let mlp = Mlp::new(mlp_layers, mlp_size, mlp_dropout);
        let output = nn::LinearConfig::new(mlp_size, NUM_CLASSES).init::<B>();
        //let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).init::<B>();
        let sigmoid = Sigmoid::new(sigmoid_strength);

        Self {
            mha,
            input,
            mlp,
            output,
            sigmoid,
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;

        let [batch_size, input_size] = x.dims();

        let attn_input = x.clone().reshape([batch_size, 1, input_size]);//.repeat(1, 3);
        //let attn_key_value = attn_input.clone();

        let attn = self.mha.forward(MhaInput::new(attn_input.clone(), attn_input.clone(), attn_input.clone()));

        let mut x = attn.context.reshape([batch_size, input_size]);

        x = self.input.forward(x);
        x = self.mlp.forward(x);
        x = self.output.forward(x);
        x = self.sigmoid.forward(x);

        x 
    }

    #[cfg(feature = "ml_train")]
    pub fn forward_classification(&self, item: KordBatch<B>) -> KordClassificationOutput<B> {
        use burn::nn::loss::MSELoss;

        let targets = item.targets;
        let output = self.forward(item.samples);

        let loss = MSELoss::default();
        let loss = loss.forward(output.clone(), targets.clone(), nn::loss::Reduction::Sum);

        // let loss = MeanSquareLoss::default();
        // let loss = loss.forward(output.clone(), targets.clone());

        // let loss = BinaryCrossEntropyLoss::default();
        // let loss = loss.forward(output.clone(), targets.clone());

        // let mut loss = FocalLoss::default();
        // loss.gamma = 2.0;
        // let loss = loss.forward(output.clone(), targets.clone());

        //let loss = loss + l1_regularization(self, 1e-4);

        // let harmonic_penalty_tensor = get_harmonic_penalty_tensor().to_device(&output.device());
        // let harmonic_loss = output.clone().matmul(harmonic_penalty_tensor).sum_dim(0).mean().mul_scalar(0.0001);

        // let loss = loss + harmonic_loss;

        KordClassificationOutput { loss, output, targets }
    }
}

#[derive(Module, Debug)]
pub struct ConvBlock<B: Backend> {
    conv: nn::conv::Conv1d<B>,
    activation: nn::ReLU,
}

impl<B: Backend> ConvBlock<B> {
    pub fn new(in_channels: usize, out_channels: usize, kernel_size: usize) -> Self {
        let conv = nn::conv::Conv1dConfig::new(in_channels, out_channels, kernel_size).with_bias(false).init::<B>();
        let activation = nn::ReLU::new();

        Self {
            conv,
            activation
        }
    }

    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.conv.forward(input);
        self.activation.forward(x)
    }
}