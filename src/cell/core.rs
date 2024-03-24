//! # Core Cell
//!
//!

use crate::activation::{derivative, Activation};
//use crate::axon::Axon;
use crate::cell::{Neuron, Nucleus};
use crate::synapse::{Synapse, SynapseIncoming};

pub(crate) struct CoreCell {
    /// Neuron value.
    pub(super) value: f32,

    /// Neuron error.
    pub(super) miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    /// All incoming and outgoing axons.
    // pub(super) synapses: (Vec<Axon>, Option<Vec<Axon>>),
    pub(super) synapses: Box<dyn Synapse>,

    _rate: f32, // TODO: Remove rate.
}

impl CoreCell {}

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
        //for axon in self.incoming_axons {
        //for axon in self.synapses.0 {
        for axon in self.synapses.get_incoming_axons() {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        //for axon in &mut self.outgoing_axons {
        //for axon in self.synapses.1.as_mut().unwrap() {
        for axon in self.synapses.get_outgoing_axons().as_mut().unwrap() {
            axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self) {
        let gradient =
            self._rate * self.miss * derivative(&(self.value as f64), &self.activation_mode) as f32;

        //for axon in &mut self.incoming_axons {
        //for axon in &mut self.synapses.0 {
        for axon in &mut self.synapses.get_incoming_axons() {
            axon.calculate_weight(&gradient);
        }
    }
}
