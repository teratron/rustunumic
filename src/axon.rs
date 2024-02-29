//! # Axon
//!
//!

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

impl<'a, T> Axon<'a, T> {
    pub(crate) fn new(inn: &'a Neuron<T>, out: &'a Neuron<T>) -> Self {
        Self {
            weight: 0. as T,
            incoming: inn,
            outgoing: out,
        }
    }
}

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
