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

use crate::cell::core::CoreCell;
use crate::cell::output::OutputCell;
use crate::cell::NeuronTrait;
use crate::float::FloatTrait;
use crate::neuron::Neurons;

pub mod activation;
pub mod loss;

mod axon;
mod cell;
mod float;
mod interface;
mod neuron;
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

    /// Function activation mode.
    activation_mode: Option<Activation>,

    /// Loss mode.
    loss_mode: Loss,

    /// Learning rate.
    rate: T,

    /// All neurons.
    neurons: Vec<Box<dyn NeuronTrait<T>>>,
    output_neurons: Neurons<T, OutputCell<T>>,
    hidden_neurons: Neurons<T, CoreCell<T>>,

    //
    is_init: bool,
    is_query: bool,
}

impl<T: FloatTrait> Rustunumic<T> {
    /// Creat new instance.
    pub fn new() -> Self {
        Self {
            bias: None,
            activation_mode: Some(Activation::ReLU),
            loss_mode: Loss::MSE,
            rate: T::DEFAULT_RATE,
            is_init: false,
            is_query: false,
            //neurons: Box::new(Vec::new()),
            neurons: Vec::new(),
            output_neurons: Neurons::new(5),
            hidden_neurons: Neurons::new(5),
        }
    }
}
