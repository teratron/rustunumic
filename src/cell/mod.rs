//! # Cell
//!
//!

use super::Float;

pub(super) mod hidden;
pub(super) mod input;
pub(super) mod output;

mod bias;
mod core;

// For types: InputCell, BiasCell, CoreCell (HiddenCell).
pub(super) trait NeuronBase<T> {
    fn get_value(&self) -> &T;
}

// For types: CoreCell (HiddenCell), OutputCell.
pub(super) trait Neuron<T>: NeuronBase<T> {
    fn get_miss(&self) -> &T;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &T);
}
