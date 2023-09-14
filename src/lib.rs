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
pub mod float;
mod interface;
mod neuron;

#[derive(Debug)]
pub struct Rustunumic<'a, T: FloatingPoint> {
    neurons: Vec<&'a Neuron<'a, T>>,
    pub rate: T,

    activation: Option<Activation>,
}

/*impl<'a, T: FloatingPoint> Rustunumic<'a, T> {
    //impl<'a> Rustunumic<'a, f64> {
    //+ std::fmt::Debug
    /// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,
            //rate: match T {  },
            activation: None,
        }
    }

    pub fn calculate_neurons(self) {
        println!("+++++++++++++++++++++++");
        for (i, neuron) in self.neurons.iter().enumerate() {
            println!("- {:#?} {:#?}", i, neuron);
        }
        println!("-----------------------");
    }
}*/

impl<'a> Rustunumic<'a, f32> {
    /// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,
            activation: None,
        }
    }
}

impl<'a> Rustunumic<'a, f64> {
    /// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,
            activation: None,
        }
    }
}

impl<T: FloatingPoint> Interface<T> for Rustunumic<'_, T> {
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
