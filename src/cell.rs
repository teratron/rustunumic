//! # Cells
//!
//!

mod bias;
mod core;
mod input;
mod output;

// For InputCell, BiasCell, CoreCell (HiddenCell).
pub(super) trait Nucleus<T> {
    fn get_value(&self) -> &T;
}

// For CoreCell (HiddenCell), OutputCell.
pub(super) trait Neuron<T>: Nucleus<T> {
    fn get_miss(&self) -> &T;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &T);
}
