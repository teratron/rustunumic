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
#![allow(unused)]

//! # Rustunumic
//!
//! Neural network library for Rust.
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

mod axon;
mod bundle;
mod cell;
mod float;
mod init;
mod network;
mod propagation;
mod properties;
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

    /// Activation mode.
    activation_mode: Option<Activation>,

    /// Loss mode.
    loss_mode: Loss,

    /// Network.
    network: Network<'a, T>,

    // State of initialization.
    is_init: bool,

    // State of query.
    is_query: bool,

    // Array of the number of neurons in each hidden layer.
    hidden_layers: Vec<usize>,
}

impl<T: Float> Rustunumic<'_, T> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            network: Network::new(),
            ..Self::default()
        }
    }
}

impl<T: Float> Default for Rustunumic<'_, T> {
    fn default() -> Self {
        Self {
            bias: None,
            rate: T::DEFAULT_RATE,
            activation_mode: Option::from(Activation::Linear),
            loss_mode: Loss::MSE,
            is_init: false,
            is_query: false,
            network: Network::new(),
            hidden_layers: vec![],
        }
    }
}
