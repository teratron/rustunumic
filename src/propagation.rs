//! # Propagation
//!
//!

use super::loss::get_total_loss;
use super::{Float, Neuron, Rustunumic};

impl<T: Float> Rustunumic<T> {
    // Forward propagation.

    /// Calculating neuron's value.
    pub(super) fn calculate_values(&mut self) {
        self.network
            .iter_mut()
            .for_each(|neuron| neuron.calculate_value())
    }

    /// Calculating and return the total error of the output neurons.
    pub(super) fn calculate_loss(&self) -> T {
        get_total_loss(
            self.output_cells
                .neurons
                .iter()
                .map(|neuron| *neuron.get_miss())
                .collect::<Vec<T>>(),
            &self.loss_mode,
        )
    }

    // Backward propagation.

    /// Calculating the error of neuron.
    pub(super) fn calculate_misses(&mut self) -> &mut Self {
        self.hidden_cells
            .neurons
            .iter_mut()
            .rev()
            .for_each(|neuron| neuron.calculate_miss());

        self
    }

    /// Update weights.
    pub(super) fn calculate_weights(&mut self) {
        self.network
            .iter_mut()
            .for_each(|neuron| neuron.calculate_weight(&self.rate));
    }
}
