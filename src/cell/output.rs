//! # Output Cell
//!
//!

use crate::cell::{Neuron, Nucleus};
use crate::cell::core::CoreCell;

pub(super) struct OutputCell {
    /// Core cell.
    core: CoreCell,

    /// Target data.
    target: f32,
}

impl OutputCell {}

impl Nucleus for OutputCell {
    fn get_value(&self) -> &f32 {
        &self.core.value
    }
}

impl Neuron for OutputCell {
    fn get_miss(&self) -> &f32 {
        &self.core.miss
    }

    fn calculate_value(&mut self) {
        self.core.calculate_value();
    }

    fn calculate_miss(&mut self) {
        self.core.miss = self.target - self.core.value;
    }

    fn calculate_weight(&mut self) {
        self.core.calculate_weight();
    }
}
