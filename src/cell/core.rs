//! # Core Cell
//!
//!

use crate::activation::{activation, get_derivative, Activation};
use crate::cell::{Neuron, Nucleus};
use crate::synapse::Synapse;

pub(super) struct CoreCell {
    /// Neuron value.
    pub(super) value: f32,

    /// Neuron error.
    pub(super) miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    /// All incoming and outgoing axons.
    pub(super) synapses: Box<dyn Synapse>,
}

impl CoreCell {
    /*fn get_activation(&mut self, mode: &Activation) {
        get_activation(&mut self.value, mode);
    }*/
}

impl Nucleus for CoreCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl Neuron for CoreCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.synapses.get_incoming_axons() {
            self.value += axon.calculate_value();
        }
        //self.get_activation(&self.activation_mode);
        activation(&mut self.value, &self.activation_mode);
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        for axon in self.synapses.get_outgoing_axons() {
            axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self, rate: &f32) {
        let gradient = *rate * self.miss * get_derivative(&mut self.value, &self.activation_mode);

        for axon in self.synapses.get_incoming_axons() {
            axon.calculate_weight(&gradient);
        }
    }
}
