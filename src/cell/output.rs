//! # Output Cell
//!
//!

use crate::Activation;
use crate::Float;

use super::CoreCell;
use super::{Neuron, NeuronBase};

pub(crate) struct OutputCell<T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: T,
}

impl<T: Float> OutputCell<T> {
    fn new(activation_mode: Activation) -> Self {
        Self {
            core: CoreCell::new(activation_mode),
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

pub(crate) struct OutputBundle<'a, T> {
    cells: Vec<OutputCell<'a, T>>,
}

impl<'a, T> OutputBundle<'a, T> {
    pub(crate) fn new(data: &[T]) -> Self {
        Self {
            cells: data
                .iter()
                .map(|v| OutputCell::new(v))
                .collect::<Vec<OutputCell<'a, T>>>(),
        }
    }
}
