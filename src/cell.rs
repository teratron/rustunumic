//! # Cells
//!
//!

mod bias;
mod core;
mod input;
mod output;

pub(super) trait Nucleus {
    fn get_value(&self) -> &f32;
}

pub(super) trait Neuron: Nucleus {
    fn get_miss(&self) -> &f32;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self, _: &f32);
}
