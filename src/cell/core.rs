//! # Core Cell
//!
//!

use crate::activation::{derivative, Activation};
use crate::axon::Axon;
use crate::cell::{CellTrait, CoreTrait};

pub(crate) struct CoreCell {
    /// Neuron value.
    pub(super) value: f32,

    /// Neuron error.
    pub(super) miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    //// All incoming axons.
    //incoming_axons: Vec<Axon>,
    /// All axons.
    pub(super) synapses: (Vec<Axon>, Option<Vec<Axon>>), //dyn Synapse,

    _rate: f32, // TODO: Remove rate.
}

/*impl CoreCell {
    // Forward propagation.
    pub(super) fn calculate_value(&mut self) {
        self.value = 0.;
        //for axon in self.incoming_axons {
        for axon in self.synapses.0 {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    pub(super) fn calculate_weight(&mut self) {
        let gradient =
            self._rate
                * self.miss
                * derivative(&(self.value as f64), &self.activation_mode) as f32;

        //for axon in &mut self.incoming_axons {
        for axon in &mut self.synapses.0 {
            axon.calculate_weight(&gradient);
        }
    }
}*/

impl CoreTrait for CoreCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl CellTrait for CoreCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        //for axon in self.incoming_axons {
        for axon in self.synapses.0 {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        //for axon in &mut self.outgoing_axons {
        for axon in self.synapses.1.as_mut().unwrap() {
            axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self) {
        let gradient =
            self._rate * self.miss * derivative(&(self.value as f64), &self.activation_mode) as f32;

        //for axon in &mut self.incoming_axons {
        for axon in &mut self.synapses.0 {
            axon.calculate_weight(&gradient);
        }
    }
}
