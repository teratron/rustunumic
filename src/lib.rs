#![crate_type = "lib"]
#![crate_name = "rustunumic"]
/*#![deny(
    rust_2021_compatibility,
    future_incompatible,
    nonstandard_style,
    clippy::all,
    clippy::doc_markdown,
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)] //unused,
*/

//! # Neural network library for Rust
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/teratron/rustunumic/examples).

// Reexported crates.
pub use activation::Activation;
pub use loss::Loss;

use crate::cell::Neuron;
use crate::float::Float;
use crate::loss::get_loss;

pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod float;
mod interface;
mod synapse;
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
pub struct Rustunumic<T> {
    /// All neurons.
    neurons: Vec<Box<dyn Neuron<T>>>,

    /// Bias neuron.
    bias: Option<bool>,

    /// Function activation mode.
    activation_mode: Option<Activation>,

    /// Loss mode.
    loss_mode: Loss,

    /// Learning rate.
    rate: T,
}

impl<T: Float> Rustunumic<T> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            //neurons: Box::new(Vec::new()),
            neurons: Vec::new(),
            bias: None,
            activation_mode: Some(Activation::ReLU),
            loss_mode: Loss::MSE,
            rate: T::DEFAULT_RATE,
        }
    }

    // Forward propagation.

    /// Calculating neuron's value.
    fn calculate_values(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_value();
        }
    }

    // Backward propagation.

    /// Calculating the error of neuron.
    fn calculate_misses(&mut self) {
        let mut n: usize = 10; //self.outgoing_axons_last_index;
        while n >= 0 {
            self.neurons[n].calculate_miss();
            n -= 1;
        }
    }

    /// Update weights.
    fn calculate_weights(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_weight(&self.rate);
        }
    }

    /// Calculating and return the total error of the output neurons.
    fn calculate_loss(&mut self) -> T {
        let mut loss = T::ZERO;
        for output in self.neurons[90..100].iter_mut() {
            loss += get_loss(output.get_miss(), &self.loss_mode);
        }
        loss /= T::from(10.);
        if self.loss_mode == Loss::RMSE {
            loss = loss.sqrt();
        }
        loss
    }
}
