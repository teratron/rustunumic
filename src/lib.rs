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

// use std::marker::PhantomData;
// use axon::Axon;
use activation::Activation;
use float::Float;
use interface::Interface;
use neuron::Neuron;

pub mod activation;
mod axon;
pub mod float;
mod interface;
mod neuron;

//struct Rate<T: Float>(T);

#[derive(Debug)]
pub struct Rustunumic<'a, T: Float> {
    neurons: Vec<&'a Neuron<'a, T>>,
    pub rate: T, // <f64 as FloatingPoint>::FloatType,
    activation: Option<Activation>,
}

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Creat new
    pub const fn new() -> Self {
        //Self.neurons = Vec::new();
        //let mut rn = Rustunumic { neurons: val, rate: val, activation: val };
        Self {
            neurons: Vec::new(),
            rate: 0.3, //.to_real(),
            activation: None,
        }
    }

    // fn to_float(v: f64) -> T {
    //     v as FloatingPoint::<T>
    // }
    /*pub fn calculate_neurons(self) {
        println!("+++++++++++++++++++++++");
        for (i, neuron) in self.neurons.iter().enumerate() {
            println!("- {:#?} {:#?}", i, neuron);
        }
        println!("-----------------------");
    }*/
}

// fn to_float(v: f64) -> impl FloatingPoint {
//     v as f32::<FloatingPoint>
// }

/*impl<'a> Rustunumic<'a, f32> {
    //// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,
            activation: None,
        }
    }
}*/

/*impl<'a> Rustunumic<'a, f64> {
    //// Creat new
    pub const fn new() -> Self {
        Rustunumic {
            neurons: Vec::new(),
            rate: 0.3,
            activation: None,
        }
    }
}*/

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
