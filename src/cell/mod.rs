//! # Cell
//!
//!

use super::Float;

pub(super) mod core;
pub(super) mod output;

mod bias;
mod input;

// For InputCell, BiasCell, CoreCell (HiddenCell).
pub(super) trait NeuronBase<T> {
    fn get_value(&self) -> &T;
}

// For CoreCell (HiddenCell), OutputCell.
pub(super) trait Neuron<T: Float>: NeuronBase<T> {
    fn get_miss(&self) -> &T;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &T);
}
