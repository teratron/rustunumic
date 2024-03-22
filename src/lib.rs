#![crate_type = "lib"]
#![crate_name = "rustunumic"]

//! # Neural network library for Rust
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/teratron/rustunumic/examples).

use crate::activation::Activation;
use crate::cell::Neuron;
use crate::loss::Loss;

pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod interface;
mod train;
mod float;

/// ## Rustunumic
///
/// **Example:**
///
/// ```rust
/// use rustunumic::Rustunumic;
///
/// let mut rn = Rustunumic::new();
/// let mut rn_f32 = Rustunumic::<f32>::new();
/// let mut rn_f64 = Rustunumic::<f64>::new();
/// ```
//#[derive(Debug)]
pub struct Rustunumic {
    /// All neurons.
    neurons: Vec<Box<dyn Neuron>>,

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
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            //neurons: Box::new(Vec::new()),
            neurons: Vec::new(),
            rate: 0.3,
            bias: None,
            activation_mode: None,
            loss_mode: Loss::MSE,
        }
    }
    
    // Forward propagation.
    fn calculate_value(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_value();
        }
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        let mut n: usize = self.outgoing_axons_last_index;
        while n >= 0 {
            self.outgoing_axons[n].calculate_miss(self);
            n -= 1;
        }
    }

    fn calculate_weight(&mut self, gradient: &f32) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_weight();
        }
    }
}
