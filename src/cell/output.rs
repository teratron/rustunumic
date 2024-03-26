//! # Output Cell
//!
//!

use crate::cell::{Neuron, Nucleus};
use crate::float::Float;

use super::core::CoreCell;

pub(super) struct OutputCell<T: Float> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: T,
}

impl<T: Float> OutputCell<T> {}

impl<T: Float> Nucleus<T> for OutputCell<T> {
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T: Float> Neuron<T> for OutputCell<T> {
    fn get_miss(&self) -> &T {
        &self.core.miss
    }

    fn calculate_value(&mut self) {
        self.core.calculate_value();
    }

    fn calculate_miss(&mut self) {
        self.core.miss = *self.target - *self.core.value;
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
