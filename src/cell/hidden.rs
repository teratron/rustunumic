//! # Hidden Cell
//!
//!

use crate::axon::AxonBundle;
use crate::Activation;
use crate::Float;

use super::CoreCell;
use super::{Neuron, Nucleus};

pub(crate) struct HiddenCell<T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Outgoing axons.
    outgoing_axons: AxonBundle<T>,
}

impl<T: Float> HiddenCell<T> {
    pub(crate) fn new(activation_mode: Activation) -> Self {
        Self {
            core: CoreCell::new(activation_mode),
            outgoing_axons: Vec::new(),
        }
    }
}

impl<T> Nucleus<T> for HiddenCell<T> {
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
        self.core.miss = T::ZERO;
        self.outgoing_axons
            .iter()
            .for_each(|a| self.core.miss += a.calculate_miss());
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
