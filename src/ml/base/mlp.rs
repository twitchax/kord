//! Multilayer Perceptron module.

use burn::{
    module::Module,
    nn::{self, LayerNorm, LayerNormConfig},
    tensor::{backend::Backend, Tensor},
};

/// Multilayer Perceptron module.
#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    linears: Vec<nn::Linear<B>>,
    norm: LayerNorm<B>,
    dropout: nn::Dropout,
    activation: nn::Relu,
}

impl<B: Backend> Mlp<B> {
    /// Create the module from the given configuration.
    pub fn new(device: &B::Device, mlp_layers: usize, mlp_size: usize, mlp_dropout: f64) -> Self {
        let mut linears = Vec::with_capacity(mlp_layers);

        for _ in 0..mlp_layers {
            let linear = nn::LinearConfig::new(mlp_size, mlp_size).init::<B>(device);
            linears.push(linear);
        }

        let norm = LayerNormConfig::new(mlp_size).init::<B>(device);
        let dropout = nn::DropoutConfig::new(mlp_dropout).init();
        let activation = nn::Relu::new();

        Self { linears, norm, dropout, activation }
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
