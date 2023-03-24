//! Multilayer Perceptron module.

use burn::{
    module::{Module, Param},
    nn,
    tensor::{backend::Backend, Tensor},
};

/// Multilayer Perceptron module.
#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    linears: Param<Vec<nn::Linear<B>>>,
    dropout: nn::Dropout,
    activation: nn::ReLU,
}

impl<B: Backend> Mlp<B> {
    /// Create the module from the given configuration.
    pub fn new(mlp_layers: usize, mlp_size: usize, mlp_dropout: f64) -> Self {
        let mut linears = Vec::with_capacity(mlp_layers);

        for _ in 0..mlp_layers {
            let linear = nn::Linear::new(&nn::LinearConfig::new(mlp_size, mlp_size));
            linears.push(linear);
        }

        Self {
            linears: Param::from(linears),
            dropout: nn::Dropout::new(&nn::DropoutConfig::new(mlp_dropout)),
            activation: nn::ReLU::new(),
        }
    }

    /// Applies the forward pass on the input tensor.
    ///
    /// # Shapes
    ///
    /// - input: `[batch_size, mlp_size]`
    /// - output: `[batch_size, mlp_size]`
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;

        for linear in self.linears.iter() {
            x = linear.forward(x);
            x = self.dropout.forward(x);
            x = self.activation.forward(x);
        }

        x
    }
}
