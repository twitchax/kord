//! Multilayer Perceptron module.

use burn::{
    nn::{self, LayerNormConfig, LayerNorm},
    tensor::{backend::Backend, Tensor}, module::Module,
};

/// Multilayer Perceptron module.
#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    linears: Vec<nn::Linear<B>>,
    norm: LayerNorm<B>,
    dropout: nn::Dropout,
    activation: nn::ReLU,
}

impl<B: Backend> Mlp<B> {
    /// Create the module from the given configuration.
    pub fn new(mlp_layers: usize, mlp_size: usize, mlp_dropout: f64) -> Self {
        let mut linears = Vec::with_capacity(mlp_layers);

        for _ in 0..mlp_layers {
            let linear = nn::LinearConfig::new(mlp_size, mlp_size).init::<B>();
            linears.push(linear);
        }

        let norm = LayerNormConfig::new(mlp_size).init::<B>();
        let dropout = nn::DropoutConfig::new(mlp_dropout).init();
        let activation = nn::ReLU::new();

        Self {
            linears,
            norm,
            dropout,
            activation
        }
    }

    /// Applies the forward pass on the input tensor.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;

        for linear in self.linears.iter() {
            //x = self.norm.forward(x);
            x = linear.forward(x);
            x = self.dropout.forward(x);
            x = self.activation.forward(x);
        }

        x
    }
}
