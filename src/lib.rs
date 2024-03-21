#![crate_type = "lib"]
#![crate_name = "rustunumic"]

use crate::activation::Activation;
use crate::cell::CellTrait;
use crate::loss::Loss;

pub mod activation;
mod axon;
mod cell;
pub mod loss;

pub struct Rustunumic {
    /// All neurons.
    neurons: Vec<Box<dyn CellTrait>>,

    /// Function activation mode.
    activation_mode: Option<Activation>,

    /// Loss mode.
    loss_mode: Loss,

    /// Learning rate.
    rate: f32,

    /// Bias neuron.
    bias: Option<bool>,
}

impl Rustunumic {
    // Forward propagation.
    fn calculate_value(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_value();
        }
    }
}
