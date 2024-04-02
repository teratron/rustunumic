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

    /// Calculating and return the total error of the output neurons.
    pub(super) fn calculate_loss(&mut self) -> T {
        let mut loss = T::ZERO;
        //for output in self.neurons[90..100].iter_mut() {
        for output in self.output_neurons.neurons.iter_mut() {
            loss += get_loss(output.get_miss(), &self.loss_mode);
        }
        loss /= self.output_neurons.number_float;
        if self.loss_mode == Loss::RMSE {
            loss = loss.sqrt();
        }
        loss
    }

    // Backward propagation.

    /// Calculating the error of neuron.
    pub(super) fn calculate_misses(&mut self) -> &mut Self {
        /*let mut n: usize = 10; //self.outgoing_axons_last_index;
        while n >= 0 {
            self.neurons[n].calculate_miss();
            n -= 1;
        }*/
        for neuron in self.hidden_neurons.neurons.iter_mut().rev() {
            neuron.calculate_miss();
        }
        self
    }

    /// Update weights.
    pub(super) fn calculate_weights(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.calculate_weight(&self.rate);
        }
    }
}
