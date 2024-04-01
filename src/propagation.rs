use super::loss::{get_loss, Loss};
use super::Float;
use super::Rustunumic;

impl<T: Float> Rustunumic<T> {
    // Forward propagation.

    /// Calculating neuron's value.
    pub(super) fn calculate_values(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_value();
        }
    }

    // Backward propagation.

    /// Calculating the error of neuron.
    pub(super) fn calculate_misses(&mut self) {
        let mut n: usize = 10; //self.outgoing_axons_last_index;
        while n >= 0 {
            self.neurons[n].calculate_miss();
            n -= 1;
        }
    }

    /// Calculating and return the total error of the output neurons.
    pub(super) fn calculate_loss(&mut self) -> T {
        let mut loss = T::ZERO;
        for output in self.neurons[90..100].iter_mut() {
            loss += get_loss(output.get_miss(), &self.loss_mode);
        }
        loss /= T::from(10.);
        if self.loss_mode == Loss::RMSE {
            loss = loss.sqrt();
        }
        loss
    }

    /// Update weights.
    pub(super) fn calculate_weights(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_weight(&self.rate);
        }
    }
}
