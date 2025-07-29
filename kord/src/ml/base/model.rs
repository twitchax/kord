//! Model definition for the Kord model.

use core::f32;

#[cfg(feature = "ml_train")]
use burn::train::MultiLabelClassificationOutput;
use burn::{
    module::Module,
    nn::{
        self,
        attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig},
    },
    tensor::{backend::Backend, Tensor},
};

use super::{INPUT_SPACE_SIZE, NUM_CLASSES};

#[cfg(feature = "ml_train")]
use crate::ml::train::data::KordBatch;

/// The Kord model.
///
/// This model is a transformer model that uses multi-head attention to classify notes from a frequency space.
#[derive(Module, Debug)]
pub struct KordModel<B: Backend> {
    mha: MultiHeadAttention<B>,
    output: nn::Linear<B>,
}

impl<B: Backend> KordModel<B> {
    /// Create the model from the given configuration.
    pub fn new(device: &B::Device, mha_heads: usize, mha_dropout: f64, _sigmoid_strength: f32) -> Self {
        let mha = MultiHeadAttentionConfig::new(INPUT_SPACE_SIZE, mha_heads).with_dropout(mha_dropout).init::<B>(device);
        let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).init::<B>(device);

        Self { mha, output }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let [batch_size, input_size] = input.dims();

        // Reshape to sequence format for attention
        let x = input.reshape([batch_size, 1, input_size]);

        // Apply multi-head attention
        let attn_output = self.mha.forward(MhaInput::new(x.clone(), x.clone(), x));

        // Flatten back to [batch_size, input_size]
        let x = attn_output.context.reshape([batch_size, input_size]);

        // Final linear layer to produce logits
        self.output.forward(x)
    }

    /// Applies the forward classification pass on the input tensor.
    #[cfg(feature = "ml_train")]
    pub fn forward_classification(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        use burn::nn::loss::BinaryCrossEntropyLossConfig;

        let targets = item.targets;
        let output = self.forward(item.samples);

        let loss = BinaryCrossEntropyLossConfig::new().with_logits(true).init(&output.device());
        let loss = loss.forward(output.clone(), targets.clone());

        // Add L1 regularization
        // let l1_reg_strength = 1e-4;
        // let l1_penalty = self.output.weight.val().abs().sum() * l1_reg_strength;
        // loss = loss + l1_penalty;

        MultiLabelClassificationOutput { loss, output, targets }
    }
}
