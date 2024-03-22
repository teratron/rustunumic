#![crate_type = "lib"]
#![crate_name = "rustunumic"]

//! # Neural network library for Rust
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/teratron/rustunumic/examples).

use crate::activation::Activation;
use crate::cell::CellTrait;
use crate::interface::Interface;
use crate::loss::Loss;

pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod interface;
mod train;

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

    // Backward propagation.
    fn calculate_miss(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_miss();
        }
        let mut n: usize = self.outgoing_axons_last_index;
        while n >= 0 {
            self.outgoing_axons[n].calculate_miss(self);
            n -= 1;
        }
    }

    fn calculate_weight(&mut self, gradient: &f32) {
        self.weight += gradient * self.incoming_cell.get_value();
    }
}

/*impl Interface<T> for Rustunumic<'_, T> {
    fn verify(&self, _input: Vec<T>, _target: Vec<T>) -> T {
        let loss: T = todo!();
        loss
    }

    fn query(&self, _input: Vec<T>) -> Vec<T> {
        let output: Vec<T> = todo!();
        output
    }

    fn train(&self, _input: Vec<T>, _target: Vec<T>) -> (usize, T) {
        let count: usize = 0;
        let loss: T = todo!();
        (count, loss)
    }
}*/
