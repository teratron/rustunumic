//! # Core Cell
//!
//!

use crate::activation::{get_activation, get_derivative, Activation, ACTIVATION_DEFAULT};
use crate::axon::AxonBundle;
use crate::Float;

//#[derive(Debug)]
pub(super) struct CoreCell<T> {
    /// Neuron value.
    pub(super) value: T,

    /// Neuron error.
    pub(super) miss: T,

    /// Function activation mode.
    activation_mode: Activation,

    /// Incoming axons.
    incoming_axons: AxonBundle<T>,
}

impl<T: Float> CoreCell<T> {
    pub(super) fn new() -> Self {
        Self {
            value: T::ZERO,
            miss: T::ZERO,
            activation_mode: ACTIVATION_DEFAULT,
            incoming_axons: Vec::new(),
        }
    }

    //////////////////////////////////////////////////////////////////////////
    // Forward propagation.
    //////////////////////////////////////////////////////////////////////////

    pub(super) fn calculate_value(&mut self) {
        self.value = T::ZERO;
        self.incoming_axons
            .iter()
            .for_each(|a| self.value += a.calculate_value());
        //self.get_activation(&self.activation_mode);
        self.value = get_activation(self.value, &self.activation_mode);
    }

    //////////////////////////////////////////////////////////////////////////
    // Backward propagation.
    //////////////////////////////////////////////////////////////////////////

    pub(super) fn calculate_weight(&mut self, rate: &T) {
        let gradient = *rate * self.miss * get_derivative(self.value, &self.activation_mode);
        self.incoming_axons
            .iter_mut()
            .for_each(|a| a.calculate_weight(&gradient));
    }
}
