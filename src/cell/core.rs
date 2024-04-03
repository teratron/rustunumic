//! # Core Cell
//!
//!

use crate::activation::{activation, derivative, Activation};
use crate::synapse::SynapseTrait;
use crate::FloatTrait;

use super::{NeuronBaseTrait, NeuronTrait};

pub(crate) struct CoreCell<T> {
    /// Neuron value.
    pub(super) value: T,

    /// Neuron error.
    pub(super) miss: T,

    /// Function activation mode.
    activation_mode: Activation,

    /// All incoming and outgoing axons.
    synapses: Box<dyn SynapseTrait<T>>,
}

impl<T> CoreCell<T> {
    /*fn get_activation(&mut self, mode: &Activation) {
        get_activation(&mut self.value, mode);
    }*/
}

impl<T> NeuronBaseTrait<T> for CoreCell<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T: FloatTrait> NeuronTrait<T> for CoreCell<T> {
    fn get_miss(&self) -> &T {
        &self.miss
    }

    // Forward propagation.

    fn calculate_value(&mut self) {
        self.value = T::ZERO;
        for axon in self.synapses.get_incoming_axons() {
            self.value += axon.calculate_value();
        }
        //self.get_activation(&self.activation_mode);
        activation(self.value, &self.activation_mode);
    }

    // Backward propagation.

    fn calculate_miss(&mut self) {
        self.miss = T::ZERO;
        for axon in self.synapses.get_outgoing_axons() {
            self.miss += axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self, rate: &T) {
        let gradient = *rate * self.miss * derivative(self.value, &self.activation_mode);
        for axon in self.synapses.get_incoming_axons().iter_mut() {
            axon.calculate_weight(&gradient);
        }
    }
}
