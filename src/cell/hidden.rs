//! # Hidden Cell
//!
//!

use super::core::CoreCell;
use super::{Float, Neuron, NeuronBase};

pub(crate) struct HiddenCell<T> {
    /// Core cell.
    core: CoreCell<T>,
}

impl<T: Float> HiddenCell<T> {
    fn new() -> Self {
        Self {
            core: CoreCell::new(),
        }
    }
}

impl<T> NeuronBase<T> for HiddenCell<T> {
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T: Float> Neuron<T> for HiddenCell<T> {
    fn get_miss(&self) -> &T {
        &self.core.miss
    }

    // Forward propagation.

    fn calculate_value(&mut self) {
        self.core.calculate_value();
    }

    // Backward propagation.

    fn calculate_miss(&mut self) {
        self.core.calculate_miss();
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
