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
//! For examples, see [examples](https://github.com/zigenzoog/rustunumic/examples).

// Reexported crates.
// pub use event_loop::{self, *};
// pub use input::{self, *};
// pub use window::{self, *};

// use axon::Axon;
use activation::Activation;
use float::FloatingPoint;
use interface::Interface;
use neuron::Neuron;

pub mod activation;
mod axon;
mod float;
mod interface;
mod neuron;

#[derive(Debug)]
pub struct Rustunumic<'a, T: FloatingPoint> {
    neurons: Vec<&'a Neuron<'a, T>>,
    rate: <f64 as FloatingPoint>::Float,

    activation: Option<Activation>,
}

impl<'a, T: FloatingPoint + std::fmt::Debug> Rustunumic<'a, T>
// where
//     T: Float + std::fmt::Debug //,
{
    /// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,

            activation: None,
        }
    }

    pub fn calc_neurons(self) {
        println!("+++++++++++++++++++++++");
        for (i, neuron) in self.neurons.iter().enumerate() {
            println!("- {:#?} {:#?}", i, neuron);
        }
        println!("-----------------------");
    }
}

impl<T: FloatingPoint> Interface<T> for Rustunumic<'_, T>
// where
//     T: Float,
{
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
