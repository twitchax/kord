//! Model definition for the Kord model.

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

#[cfg(all(feature = "ml_train", feature = "ml_target_folded_bass"))]
use super::{PITCH_CLASS_COUNT, TARGET_FOLDED_BASS_NOTE_OFFSET, TARGET_FOLDED_BASS_OFFSET};

#[cfg(feature = "ml_train")]
use crate::ml::train::data::KordBatch;

/// The Kord model.
///
/// This model is a transformer model that uses multi-head attention to classify notes from a frequency space.
#[derive(Module, Debug)]
pub struct KordModel<B: Backend> {
    mha: MultiHeadAttention<B>,

    trunk_in: nn::Linear<B>,
    trunk_dropout: nn::Dropout,
    trunk_out: nn::Linear<B>,

    output: nn::Linear<B>,
}

impl<B: Backend> KordModel<B> {
    /// Create the model from the given configuration.
    pub fn new(device: &B::Device, mha_heads: usize, mha_dropout: f64, trunk_max_hidden_size: usize, _sigmoid_strength: f32) -> Self {
        let mha = MultiHeadAttentionConfig::new(INPUT_SPACE_SIZE, mha_heads).with_dropout(mha_dropout).init::<B>(device);

        let trunk_hidden = INPUT_SPACE_SIZE.min(trunk_max_hidden_size);

        let trunk_in = nn::LinearConfig::new(INPUT_SPACE_SIZE, trunk_hidden).with_bias(true).init::<B>(device);
        let trunk_dropout = nn::DropoutConfig::new(mha_dropout).init();
        let trunk_out = nn::LinearConfig::new(trunk_hidden, INPUT_SPACE_SIZE).with_bias(true).init::<B>(device);

        let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).with_bias(true).init::<B>(device);

        Self {
            mha,
            trunk_in,
            trunk_dropout,
            trunk_out,
            output,
        }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        use burn::tensor::activation::gelu;

        let [batch_size, input_size] = input.dims();

        // Reshape to sequence format for attention
        let x = input.reshape([batch_size, 1, input_size]);

        // Apply multi-head attention
        let attn_output = self.mha.forward(MhaInput::new(x.clone(), x.clone(), x));

        // Flatten back to [batch_size, input_size]
        let x = attn_output.context.reshape([batch_size, input_size]);

        // Light MLP trunk before the final output head.
        let x = self.trunk_in.forward(x);
        let x = gelu(x);
        let x = self.trunk_dropout.forward(x);
        let x = self.trunk_out.forward(x);

        // Final linear layer to produce logits
        self.output.forward(x)
    }

    /// Applies the forward classification pass on the input tensor.
    #[cfg(all(feature = "ml_train", feature = "ml_target_full"))]
    pub fn forward_classification(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        use burn::nn::loss::BinaryCrossEntropyLossConfig;

        let logits = self.forward(item.samples);
        let targets = item.targets;

        let loss = BinaryCrossEntropyLossConfig::new().with_logits(true).init(&logits.device()).forward(logits.clone(), targets.clone());

        MultiLabelClassificationOutput { loss, output: logits, targets }
    }

    /// Applies the forward classification pass when only the folded target is enabled.
    #[cfg(all(feature = "ml_train", feature = "ml_target_folded"))]
    pub fn forward_classification(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        use burn::nn::loss::BinaryCrossEntropyLossConfig;

        let logits = self.forward(item.samples);
        let targets = item.targets;

        let loss = BinaryCrossEntropyLossConfig::new().with_logits(true).init(&logits.device()).forward(logits.clone(), targets.clone());

        MultiLabelClassificationOutput { loss, output: logits, targets }
    }

    /// Applies the forward classification pass when the folded-bass target is enabled.
    #[cfg(all(feature = "ml_train", feature = "ml_target_folded_bass"))]
    pub fn forward_classification(&self, item: KordBatch<B>) -> MultiLabelClassificationOutput<B> {
        use burn::nn::loss::{BinaryCrossEntropyLossConfig, CrossEntropyLossConfig};

        let logits = self.forward(item.samples);
        let targets = item.targets;
        let batch = logits.dims()[0];

        let device = logits.device();
        let bce_loss = BinaryCrossEntropyLossConfig::new().with_logits(true).init(&device);
        let ce_loss = CrossEntropyLossConfig::new().init(&device);

        let bass_start = TARGET_FOLDED_BASS_OFFSET;
        let bass_end = bass_start + PITCH_CLASS_COUNT;
        let note_start = TARGET_FOLDED_BASS_NOTE_OFFSET;
        let note_end = note_start + PITCH_CLASS_COUNT;

        let bass_logits = logits.clone().slice([0..batch, bass_start..bass_end]);
        let note_logits = logits.clone().slice([0..batch, note_start..note_end]);

        let note_targets = targets.clone().slice([0..batch, note_start..note_end]);
        let bass_targets_hot = targets.clone().slice([0..batch, bass_start..bass_end]);
        let bass_targets = bass_targets_hot.argmax(1).squeeze();

        let note_loss = bce_loss.forward(note_logits, note_targets);
        let categorical_loss = ce_loss.forward(bass_logits, bass_targets);
        let loss = note_loss + categorical_loss;

        MultiLabelClassificationOutput { loss, output: logits, targets }
    }
}
