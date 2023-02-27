//! Model definition for the Kord model.

use core::f32;
use std::ops::Deref;

use burn::{
    module::{Module, Param},
    nn::{self},
    tensor::{
        backend::{Backend},
        Tensor, Distribution, ElementConversion, Data
    },
    config::Config,
};

use super::{helpers::Sigmoid, mlp::Mlp, INPUT_SPACE_SIZE, NUM_CLASSES};

use crate::{analyze::base::get_frequency_bins, core::note::ALL_PITCH_NOTES};

#[cfg(feature = "ml_train")]
use crate::ml::train::{helpers::{KordClassificationOutput, MeanSquareLoss}, data::KordBatch};

#[derive(Module, Debug)]
pub struct KordModel<B: Backend> {
    //conv: Param<HarmonicConvolution<B>>,
    input: Param<nn::Linear<B>>,
    mlp: Param<Mlp<B>>,
    output: Param<nn::Linear<B>>,
    sigmoid: Sigmoid,
}

impl<B: Backend> KordModel<B> {
    pub fn new(mlp_layers: usize, mlp_size: usize, mlp_dropout: f64, sigmoid_strength: f32) -> Self {
        //let conv = HarmonicConvolution::new::<INPUT_SPACE_SIZE>(&HarmonicConvolutionConfig::new(INPUT_SPACE_SIZE));

        let input = nn::Linear::new(&nn::LinearConfig::new(INPUT_SPACE_SIZE, mlp_size));
        let mlp = Mlp::new(mlp_layers, mlp_size, mlp_dropout);
        let output = nn::Linear::new(&nn::LinearConfig::new(mlp_size, NUM_CLASSES));
        let sigmoid = Sigmoid::new(sigmoid_strength);

        Self {
            //conv: Param::new(conv),
            input: Param::new(input),
            mlp: Param::new(mlp),
            output: Param::new(output),
            sigmoid,
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;

        //x = self.conv.forward(x);
        x = self.input.forward(x);
        x = self.mlp.forward(x);
        x = self.output.forward(x);
        x = self.sigmoid.forward(x);

        x
    }

    #[cfg(feature = "ml_train")]
    pub fn forward_classification(&self, item: KordBatch<B>) -> KordClassificationOutput<B> {
        

        

        let targets = item.targets;
        let output = self.forward(item.samples);

        let loss = MeanSquareLoss::default();
        let loss = loss.forward(&output, &targets);

        // let loss = BinaryCrossEntropyLoss::default();
        // let loss = loss.forward(&output, &targets);

        KordClassificationOutput { loss, output, targets }
    }
}

// Harmonic Convolution.

/// Configuration to create a [Linear](Linear) layer.
#[derive(Config, Debug)]
pub struct HarmonicConvolutionConfig {
    /// The size of the input features.
    pub d_input: usize,
    /// If a bias should be applied during the linear transformation.
    #[config(default = true)]
    pub bias: bool,
}

#[derive(Module, Debug)]
pub struct HarmonicConvolution<B: Backend> {
    config: HarmonicConvolutionConfig,
    mask: Data<B::Elem, 2>,
    weight: Param<Tensor<B, 2>>,
    bias: Param<Option<Tensor<B, 1>>>,
}

impl<B: Backend> HarmonicConvolution<B> {
    /// Create the module from the given configuration.
    pub fn new<const D: usize>(config: &HarmonicConvolutionConfig) -> Self {
        let k = f64::sqrt(1.0 / config.d_input as f64);
        let distribution = Distribution::Uniform((-1.0 * k).to_elem(), k.to_elem());

        let mut masks = vec![];
        let mut weights = vec![];

        for (_, (low, high)) in get_frequency_bins(&ALL_PITCH_NOTES.iter().skip(23).take(62).cloned().collect::<Vec<_>>())  {
            let weight = Tensor::random([config.d_input], distribution).reshape([config.d_input, 1]);

            let mut mask = [0.0f32; D];

            for k in 1..15 {
                let low_harmonic = (low * k as f32).round() as usize;
                let high_harmonic = (high * k as f32).round() as usize;

                if high_harmonic >= config.d_input {
                    break;
                }
                
                for k in low_harmonic..high_harmonic {
                    mask[k] = 1.0;
                }
            }

            let mask = Tensor::<B, 1>::from_data(Data::<f32, 1>::from(mask).convert()).reshape([config.d_input, 1]);

            masks.push(mask);
            weights.push(weight);
        }

        let mask = Tensor::cat(masks, 1).to_data();
        let weight = Tensor::cat(weights, 1);

        let bias = match config.bias {
            true => Some(Tensor::random([60], distribution)),
            false => None,
        };

        Self {
            config: config.clone(),
            mask,
            weight: Param::new(weight),
            bias: Param::new(bias),
        }
    }
    
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let device = input.device();

        let mask = Tensor::from_data(self.mask.clone()).to_device(&device);
        let matrix = mask.mul(&self.weight).unsqueeze();
        let output = input.matmul(&matrix);

        match self.bias.deref() {
            Some(bias) => output + bias.unsqueeze(),
            None => output,
        }
    }
}
