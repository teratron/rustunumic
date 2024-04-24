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
)] //unused,*/

//! # Neural network library for Rust
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/teratron/rustunumic/examples).

// Reexported crates.
pub use activation::Activation;
pub use loss::Loss;

use crate::bundle::Bundle;
use crate::cell::hidden::HiddenCell;
use crate::cell::input::InputCell;
use crate::cell::output::OutputCell;
use crate::cell::{Neuron, Nucleus};
use crate::float::Float;
use crate::network::Network;

// Reexported modules.
pub mod activation;
pub mod loss;

mod _interface;
mod axon;
mod bundle;
mod cell;
mod float;
mod init;
mod network;
mod propagation;
mod query;
mod train;
mod verify;

/// Rustunumic
///
/// # Example
///
/// ```rust
/// use rustunumic::Rustunumic;
///
/// let mut rn = Rustunumic::new();
/// let mut rn_f32 = Rustunumic::<f32>::new();
/// let mut rn_f64 = Rustunumic::<f64>::new();
/// ```
//#[derive(Debug)]
pub struct Rustunumic<'a, T> {
    /// Bias neuron.
    bias: Option<bool>,

    /// Learning rate.
    rate: T,

    /// Loss mode.
    loss_mode: Loss,

    /// Network.
    network: Network<'a, T>,

    // State.
    pub(crate) is_init: bool,
    pub(crate) is_query: bool,
}

impl<T: Float> Rustunumic<'_, T> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            bias: None,
            loss_mode: Loss::MSE,
            rate: T::DEFAULT_RATE,
            is_init: false,
            is_query: false,
            network: Network::new(),
            //network: Vec::new(),
            //input: InputBundle::new(&[]),
            //output: Bundle::<T, OutputCell<T>>::new(5),
            //hidden: Bundle::<T, HiddenCell<T>>::new(5),
        }
    }
}
