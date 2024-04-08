//! # Propagation
//!
//!

use super::loss::{get_loss, Loss};
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
        let mut loss = T::ZERO;
        self.output_cells
            .neurons
            .iter()
            .for_each(|neuron| loss += get_loss(neuron.get_miss(), &self.loss_mode));
        loss /= self.output_cells.number_float;
        if self.loss_mode == Loss::RMSE {
            loss = loss.sqrt();
        }
        loss
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
