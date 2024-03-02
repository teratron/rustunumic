//! # Axon
//!
//!

use crate::float::Float;
use crate::neuron::Neuron;

#[derive(Debug)]
pub struct Axon<'a, T> {
    /// Axon weight.
    pub weight: T,

    /// Incoming synapse.
    pub(crate) incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    pub(crate) outgoing: &'a Neuron<'a, T>,
}

impl<'a, T: Float> Axon<'a, T> {
    pub(crate) fn new(inn: &'a Neuron<T>, out: &'a Neuron<T>) -> Self {
        Self {
            weight: T::ZERO,
            incoming: inn,
            outgoing: out,
        }
    }
}

enum Synapse {
    Incoming,
    Outgoing,
}

/*impl<T: Float> Default for T {
    fn default() -> Self {
        T::ZERO
    }
}*/

/*impl<'a> Axon<'a, f64> {
    pub(crate) fn new(inn: &'a Neuron<f64>, out: &'a Neuron<f64>) -> Self {
        Self {
            weight: 0.,
            incoming: inn,
            outgoing: out
        }
    }

    pub(crate) fn back(&mut self) -> f64 {
        self.incoming.value * self.weight
    }
}*/
