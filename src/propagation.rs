//! # Propagation
//!
//!

use super::{Float, Neuron, Rustunumic};
use super::loss::{get_loss, Loss};

impl<T: Float> Rustunumic<T> {
    // Forward propagation.

    /// Calculating neuron's value.
    pub(super) fn calculate_values(&mut self) {
        /*for neuron in self.network.iter_mut() {
            neuron.calculate_value();
        }*/
        self.network
            .iter_mut()
            .for_each(|n| n.calculate_value())
    }

    /// Calculating and return the total error of the output neurons.
    pub(super) fn calculate_loss(&mut self) -> T {
        let mut loss = T::ZERO;
        for output in self.output.neurons.iter_mut() {
            loss += get_loss(output.get_miss(), &self.loss_mode);
        }

        self.output.neurons
            .iter_mut()
            .reduce(|acc, e| acc + get_loss(e.get_miss(), &self.loss_mode))
            .unwrap();

        loss /= self.output.number_float;
        if self.loss_mode == Loss::RMSE {
            loss = loss.sqrt();
        }
        loss
    }

    // Backward propagation.

    /// Calculating the error of neuron.
    pub(super) fn calculate_misses(&mut self) -> &mut Self {
        /*for neuron in self.hidden.neurons.iter_mut().rev() {
            neuron.calculate_miss();
        }*/
        self.hidden.neurons
            .iter_mut()
            .rev()
            .for_each(|n| n.calculate_miss());
        self
    }

    /// Update weights.
    pub(super) fn calculate_weights(&mut self) {
        /*for neuron in self.network.iter_mut() {
            neuron.calculate_weight(&self.rate);
        }*/
        self.network
            .iter_mut()
            .for_each(|n| n.calculate_weight(&self.rate));
    }
}
