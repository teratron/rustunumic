//! # Cells
//!
//!

mod bias;
mod core;
mod hidden;
mod input;
mod output;

pub(super) trait CoreTrait {
    fn get_value(&self) -> &f32;
}

pub(super) trait CellTrait: CoreTrait {
    fn get_miss(&self) -> &f32;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self);
}
