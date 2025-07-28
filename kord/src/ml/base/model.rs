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

use super::{helpers::Sigmoid, INPUT_SPACE_SIZE, NUM_CLASSES};

#[cfg(feature = "ml_train")]
use crate::ml::train::data::KordBatch;

/// The Kord model.
///
/// This model is a transformer model that uses multi-head attention to classify notes from a frequency space.
#[derive(Module, Debug)]
pub struct KordModel<B: Backend> {
    mha: MultiHeadAttention<B>,
    output: nn::Linear<B>,
    sigmoid: Sigmoid<B>,
}

impl<B: Backend> KordModel<B> {
    /// Create the model from the given configuration.
    pub fn new(device: &B::Device, mha_heads: usize, mha_dropout: f64, sigmoid_strength: f32) -> Self {
        let mha = MultiHeadAttentionConfig::new(INPUT_SPACE_SIZE, mha_heads).with_dropout(mha_dropout).init::<B>(device);
        let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).init::<B>(device);
        let sigmoid = Sigmoid::new(device, sigmoid_strength);

        Self { mha, output, sigmoid }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = input;

        // Perform the multi-head attention transformer forward pass.
        let [batch_size, input_size] = x.dims();
        let attn_input = x.clone().reshape([batch_size, 1, input_size]);
        let attn = self.mha.forward(MhaInput::new(attn_input.clone(), attn_input.clone(), attn_input));

        // Reshape the output to remove the sequence dimension.
        let mut x = attn.context.reshape([batch_size, input_size]);

        // Perform the final linear layer to map to the output dimensions.
        x = self.output.forward(x);

        // Apply the sigmoid function to the output to achieve multi-classification.
        x = self.sigmoid.forward(x);

        x
    }

    /// Applies the forward classification pass on the input tensor.
    #[cfg(feature = "ml_train")]
    pub fn forward_classification(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        use burn::nn::loss::BinaryCrossEntropyLossConfig;

        let targets = item.targets;
        let output = self.forward(item.samples);

        let loss = BinaryCrossEntropyLossConfig::new().with_logits(false).init(&output.device());
        let mut loss = loss.forward(output.clone(), targets.clone());

        // Add L1 regularization
        // let l1_reg_strength = 1e-4;
        // let l1_penalty = self.output.weight.val().abs().sum() * l1_reg_strength;
        // loss = loss + l1_penalty;

        MultiLabelClassificationOutput { loss, output, targets }
    }
}
