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

mod activation;
mod neuron;
mod axon;

use neuron::Neuron;
use axon::Axon;

// Float
pub trait Float {}
impl Float for f32 {}
impl Float for f64 {}

#[derive(Debug)]
pub struct Rustunumic<T>
where
    T: Float,
{
    neurons: Vec<Neuron<T>>,
    axons: Vec<Axon<T>>,
}

impl<T> Rustunumic<T>
where
    T: Float,
{
    /// Creat new
    pub fn new() -> Self {
        let _v = vec![0., 2., 4., 6.];
        Rustunumic {
            neurons: Vec::new(),
            axons: Vec::new(),
        }
    }
}
