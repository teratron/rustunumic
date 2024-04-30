//! # Cell
//!
//!

use std::fmt::Debug;

use core::CoreCell;

use crate::Activation;

pub(super) mod hidden;
pub(super) mod input;
pub(super) mod output;

mod bias;
mod core;

// For types: HiddenCell, InputCell, BiasCell.
pub(super) trait Nucleus<T>: Debug {
    fn get_value(&self) -> &T;
}

// For types: HiddenCell, OutputCell.
pub(super) trait Neuron<T>: Nucleus<T> {
    fn news(activation_mode: Activation) -> Box<Self>;
    fn get_miss(&self) -> &T;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &T);
}

/*pub(super) enum CellKind<T> {
    Input(T),
    BackfedInput,
    NoisyInput,

    Hidden(HiddenCell<T>),
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias,

    Output(OutputCell<T>),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}

impl<T: Float> CellKind<T> {}*/
