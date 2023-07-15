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

//! # Simple neural network library for Rust.
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
mod neuron;

use activation::Activation;

#[derive(Debug)]
pub struct Rustunumic {
    //activation: Activation,
}

impl Rustunumic {
    /// Creat new
    pub fn new() -> Self {
        let _v = vec![0, 2, 4, 6];
        Rustunumic {
            //activation: Activation::SIGMOID,
        }
    }

    /*pub fn sigmoid(&self) -> Activation {
        Activation::SIGMOID
    }*/

    pub const SIGMOID: Activation = Activation::SIGMOID;
}

pub trait Interface {
    fn verify(&self, input: Vec<f64>, target: Vec<f64>) -> f64;
    fn query(&self, input: Vec<f64>) -> Vec<f64>;
    fn train(&self, input: Vec<f64>, target: Vec<f64>) -> (f64, usize);
}

impl Interface for Rustunumic {
    fn verify(&self, _input: Vec<f64>, _target: Vec<f64>) -> f64 {
        0. // TODO:
    }

    fn query(&self, _input: Vec<f64>) -> Vec<f64> {
        vec![1., 2., 3.] // TODO:
    }

    fn train(&self, _input: Vec<f64>, _target: Vec<f64>) -> (f64, usize) {
        (0.5, 42) // TODO:
    }
}
