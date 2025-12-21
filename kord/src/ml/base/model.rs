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
    norm1: nn::LayerNorm<B>,

    trunk_in: nn::Linear<B>,
    trunk_dropout: nn::Dropout,
    trunk_out: nn::Linear<B>,
    norm2: nn::LayerNorm<B>,

    output: nn::Linear<B>,

    // Store chunk dimensions for forward pass
    num_chunks: usize,
    chunk_size: usize,
}

impl<B: Backend> KordModel<B> {
    /// Create the model from the given configuration.
    pub fn new(device: &B::Device, mha_heads: usize, dropout: f64, trunk_hidden_size: usize, _sigmoid_strength: f32) -> Self {
        // Calculate chunk dimensions based on number of heads
        // Each head gets one chunk to attend to
        let num_chunks = mha_heads;
        let chunk_size = INPUT_SPACE_SIZE / mha_heads;

        let mha = MultiHeadAttentionConfig::new(chunk_size, mha_heads).with_dropout(dropout).init::<B>(device);
        let norm1 = nn::LayerNormConfig::new(INPUT_SPACE_SIZE).init(device);

        let trunk_in = nn::LinearConfig::new(INPUT_SPACE_SIZE, trunk_hidden_size).with_bias(true).init::<B>(device);
        let trunk_dropout = nn::DropoutConfig::new(dropout).init();
        let trunk_out = nn::LinearConfig::new(trunk_hidden_size, INPUT_SPACE_SIZE).with_bias(true).init::<B>(device);
        let norm2 = nn::LayerNormConfig::new(INPUT_SPACE_SIZE).init(device);

        let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).with_bias(true).init::<B>(device);

        Self {
            mha,
            norm1,
            trunk_in,
            trunk_dropout,
            trunk_out,
            norm2,
            output,
            num_chunks,
            chunk_size,
        }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        use burn::tensor::activation::gelu;

        let [batch_size, input_size] = input.dims();

        // Reshape to multi-token sequence format for attention
        // Split input into chunks so attention can attend between different frequency regions
        // Each head gets its own chunk (num_chunks = mha_heads)
        let x = input.clone().reshape([batch_size, self.num_chunks, self.chunk_size]);

        // Apply multi-head attention across the chunk sequence
        let attn_output = self.mha.forward(MhaInput::new(x.clone(), x.clone(), x));

        // Flatten back to [batch_size, input_size]
        let attn_out = attn_output.context.reshape([batch_size, input_size]);

        // Post-norm after attention with residual (normalize over full input_size)
        let x = self.norm1.forward(input + attn_out);

        // Light MLP trunk with residual connection and post-norm before the final output head.
        let trunk_in = self.trunk_in.forward(x.clone());
        let trunk_in = gelu(trunk_in);
        let trunk_in = self.trunk_dropout.forward(trunk_in);
        let trunk_out = self.trunk_out.forward(trunk_in);
        let x = self.norm2.forward(x + trunk_out);

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
