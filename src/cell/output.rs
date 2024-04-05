//! # Output Cell
//!
//!

use super::{Float, Neuron, NeuronBase};
use super::core::CoreCell;

pub(crate) struct OutputCell<T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: T,
}

impl<T: Float> OutputCell<T> {
    fn new() -> Self {
        Self {
            core: CoreCell::new(),
            target: T::ZERO,
        }
    }
}

impl<T> NeuronBase<T> for OutputCell<T> {
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T: Float> Neuron<T> for OutputCell<T> {
    fn get_miss(&self) -> &T {
        &self.core.miss
    }

    // Forward propagation.

    fn calculate_value(&mut self) {
        self.core.calculate_value();
        self.calculate_miss();
    }

    // Backward propagation.

    fn calculate_miss(&mut self) {
        self.core.miss = self.target - self.core.value;
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
