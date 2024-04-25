//! # Output Cell
//!
//!

use std::fmt::{Debug, Formatter, Result};

use crate::Activation;
use crate::Float;

use super::CoreCell;
use super::{Neuron, Nucleus};

pub(crate) struct OutputCell<'a, T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: &'a T,
}

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

impl<T> Nucleus<T> for OutputCell<'_, T> {
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

impl<T> Debug for OutputCell<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("OutputCell")
            .field("core", &self.core)
            .field("target", &self.target)
            .finish()
    }
}
