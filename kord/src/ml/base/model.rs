//! Model definition for the Kord model.

use core::f32;

#[cfg(feature = "ml_train")]
use burn::train::MultiLabelClassificationOutput;
use burn::{
    module::Module,
    nn::{
        self,
        attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig},
        Gelu,
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
    embed: nn::Linear<B>,
    ln1: nn::LayerNorm<B>,
    ln2: nn::LayerNorm<B>,
    attn_dropout: nn::Dropout,
    ffn_in: nn::Linear<B>,
    ffn_out: nn::Linear<B>,

    mha: MultiHeadAttention<B>,
    output: nn::Linear<B>,
}

impl<B: Backend> KordModel<B> {
    /// Create the model from the given configuration.
    pub fn new(device: &B::Device, mha_heads: usize, mha_dropout: f64, _sigmoid_strength: f32) -> Self {
        let embed = nn::LinearConfig::new(1, INPUT_SPACE_SIZE).init(device);

        let ln1 = nn::LayerNormConfig::new(INPUT_SPACE_SIZE).init(device);
        let ln2 = nn::LayerNormConfig::new(INPUT_SPACE_SIZE).init(device);
        let attn_dropout = nn::DropoutConfig::new(mha_dropout).init();

        let ffn_in = nn::LinearConfig::new(INPUT_SPACE_SIZE, 4 * INPUT_SPACE_SIZE).init(device);
        let ffn_out = nn::LinearConfig::new(4 * INPUT_SPACE_SIZE, INPUT_SPACE_SIZE).init(device);

        let mha = MultiHeadAttentionConfig::new(INPUT_SPACE_SIZE, mha_heads).with_dropout(mha_dropout).init::<B>(device);
        let output = nn::LinearConfig::new(INPUT_SPACE_SIZE, NUM_CLASSES).with_bias(true).init::<B>(device);

        Self {
            embed,
            ln1,
            ln2,
            attn_dropout,
            ffn_in,
            ffn_out,
            mha,
            output,
        }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        // x: [B, F] scalars
        let [b, f] = input.dims();

        // [B, F] -> [B, F, 1] -> per-bin embed to [B, F, d_model]
        let x = input.reshape([b, f, 1]);
        let mut h = self.embed.forward(x); // [B, F, d_model]

        // (Optional but helpful) add sinusoidal positional encoding over F
        let pe = sinusoidal_pe::<B>(f, INPUT_SPACE_SIZE, &h.device()); // [F, d_model]
        let pe = pe.unsqueeze::<3>().expand([b, f, INPUT_SPACE_SIZE]);
        h = h + pe;

        // Pre-attn norm
        let h_norm = self.ln1.forward(h.clone()); // [B, F, d_model]

        // Burn’s MHA expects [B, T, d_model]; use h_norm as Q=K=V
        let attn = self.mha.forward(nn::attention::MhaInput::new(h_norm.clone(), h_norm.clone(), h_norm));

        // attn.context: [B, F, d_model]
        let ctx = self.attn_dropout.forward(attn.context);

        // Residual + FFN block
        let h2 = h + ctx; // residual
        let h2n = self.ln2.forward(h2.clone());
        let ff = Gelu::new().forward(self.ffn_in.forward(h2n));
        let ff = self.ffn_out.forward(ff);
        let h3 = h2 + ff; // residual

        // Pool tokens (frequency bins) -> [B, d_model]
        let pooled = h3.mean_dim(1).squeeze(1);

        // Classify -> logits [B, 128]
        self.output.forward(pooled)
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

fn sinusoidal_pe<B: Backend>(f: usize, d_model: usize, device: &B::Device) -> Tensor<B, 2> {
    let range1 = (0..f as u32).map(|x| x as f32).collect::<Vec<_>>();
    let range2 = (0..(d_model / 2) as u32).map(|x| x as f32).collect::<Vec<_>>();

    let pos = Tensor::<B, 1>::from(&range1[..]).reshape([f, 1]).to_device(device);
    let i = Tensor::<B, 1>::from(&range2[..]).to_device(device);

    let denom = Tensor::<B, 1>::from_floats([10000.0], device).powf(i * 2.0 / Tensor::from_floats([d_model as f32], device));
    let angle = pos / denom.unsqueeze::<2>().expand([f, d_model / 2]);

    let sin = angle.clone().sin();
    let cos = angle.cos();

    Tensor::cat(vec![sin, cos], 1)
}
