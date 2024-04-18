//! # Output Cell
//!
//!

use crate::Activation;
use crate::Float;

use super::CoreCell;
use super::{Neuron, NeuronBase};

pub(crate) struct OutputCell<'a, T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: &'a T,
}

/*impl<T: Float> Default for OutputCell<'_, T> {
    fn default() -> Self {
        Self {
            core: CoreCell::new(Activation::Linear),
            target: &T::from(0.),
        }
    }
}*/

impl<'a, T: Float> OutputCell<'a, T> {
    pub(crate) fn new(activation_mode: Activation, target: &'a T) -> Self {
        Self {
            core: CoreCell::new(activation_mode),
            target,
        }
    }

    pub(crate) fn set_target(&mut self, target: &'a T) {
        self.target = target;
    }
}

impl<T> NeuronBase<T> for OutputCell<'_, T> {
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T: Float> Neuron<T> for OutputCell<'_, T> {
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
        self.core.miss = *self.target - self.core.value;
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
