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

//! # Neural network library for Rust.
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/teratron/rustunumic/examples).

// Reexported crates.
// pub use event_loop::{self, *};
// pub use input::{self, *};
// pub use window::{self, *};

use std::fmt::Debug;

use crate::activation::Activation;
use crate::float::Float;
use crate::interface::Interface;
use crate::loss::Loss;
use crate::neuron::Neuron;

pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod float;
mod interface;
mod neuron;

/// ## Rustunumic.
///
/// **Example:**
///
/// ```rust
/// use rustunumic::Rustunumic;
///
/// let mut rn = Rustunumic::new();
/// ```
//#[derive(Debug)]
pub struct Rustunumic<'a, T: Float> {
    //neurons: Box<Vec<&'a dyn Neuron<'a, T>>>,
    neurons: Vec<&'a dyn Neuron<'a, T>>,
    rate: T,
    activation: Option<Activation>,
    loss: Option<Loss>,
    bias: Option<bool>,
}

/* impl<'a, T: Float> Rustunumic<'a, T> {

} */

impl<'a> Rustunumic<'a, f64> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            //neurons: Box::new(Vec::new()),
            neurons: Vec::new(),
            rate: 0.3,
            bias: None,
            activation: None,
            loss: None,
        }
    }

    // Propagation
    pub fn calculate_neurons(&mut self) {
        for neuron in self.neurons.iter_mut() {
            //println!("- {:#?} {:#?}", i, neuron);
            //neuron.calculate_value();
        }
    }
}

impl<T: Float> Interface<T> for Rustunumic<'_, T> {
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
}
