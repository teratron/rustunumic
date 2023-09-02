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

pub mod activation;
mod axon;
mod float;
mod interface;
mod neuron;

// use axon::Axon;
use activation::Activation;
use float::Float;
use interface::Interface;
use neuron::Neuron;

#[derive(Debug)]
pub struct Rustunumic<'a, T>
where
    T: Float,
{
    neurons: Vec<&'a Neuron<'a, T>>,
    activation: Option<Activation>,
    rate: T,
}

impl<'a, T> Rustunumic<'a, T>
where
    T: Float + std::fmt::Debug,
{
    /// Creat new
    pub fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            activation: None,
            rate: (0.3).to_real(),
        }
    }

    pub fn calc_neurons(&self) {
        for i in &self.neurons {
            print!("{:?}", i)
        }
    }
}

impl<T> Interface<T> for Rustunumic<'_, T>
where
    T: Float,
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
