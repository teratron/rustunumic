//! # Cell
//!
//!

pub mod core;
pub mod output;

mod bias;
mod input;

// For InputCell, BiasCell, CoreCell (HiddenCell).
pub(super) trait NeuronBaseTrait<T> {
    fn get_value(&self) -> &T;
}

// For CoreCell (HiddenCell), OutputCell.
pub(super) trait NeuronTrait<T>: NeuronBaseTrait<T> {
    fn get_miss(&self) -> &T;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &T);
}
