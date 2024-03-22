//! # Core Cell
//!
//!

use crate::activation::{derivative, Activation};
use crate::axon::Axon;

pub(crate) struct CoreCell {
    /// Neuron value.
    pub(super) value: f32,

    /// Neuron error.
    pub(super) miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,

    synapses: (Vec<Axon>, Option<Vec<Axon>>), //dyn Synapse,

                                              //_rate: f32,
}

impl CoreCell {
    // Forward propagation.
    pub(super) fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    pub(super) fn calculate_weight(&mut self) {
        let gradient =
            self._rate * self.miss * derivative(&(self.value as f64), &self.activation_mode) as f32;

        for axon in &mut self.incoming_axons {
            axon.calculate_weight(&gradient);
        }
    }
}
