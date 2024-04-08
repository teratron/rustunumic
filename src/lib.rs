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

use crate::cell::hidden::HiddenCell;
use crate::cell::output::OutputCell;
use crate::cell::{Neuron, NeuronBase};
use crate::float::Float;
use crate::network::Network;

// Reexported modules.
pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod float;
mod init;
mod interface;
mod network;
mod propagation;
mod query;
mod synapse;
mod train;
mod verify;

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
    /// Bias neuron.
    bias: Option<bool>,

    /// Learning rate.
    rate: T,

    /// Loss mode.
    loss_mode: Loss,

    /// All neurons.
    network: Vec<Box<dyn Neuron<T>>>,

    /// Input neurons.
    //pub(crate) input_cells: Network<T, InputCell<T>>,

    /// Output neurons.
    pub(crate) output_cells: Network<T, OutputCell<T>>,

    /// Hidden neurons.
    pub(crate) hidden_cells: Network<T, HiddenCell<T>>,

    // State.
    pub(crate) is_init: bool,
    pub(crate) is_query: bool,
}

impl<T: Float> Rustunumic<T> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            bias: None,
            loss_mode: Loss::MSE,
            rate: T::DEFAULT_RATE,
            is_init: false,
            is_query: false,
            network: Vec::new(),
            //input_cells: Network::<T, InputCell<T>>::new(5),
            output_cells: Network::<T, OutputCell<T>>::new(5),
            hidden_cells: Network::<T, HiddenCell<T>>::new(5),
        }
    }
}
