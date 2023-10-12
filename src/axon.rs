//! # Axon
//!
//!

use crate::neuron::{CellKind, Neuron};

#[derive(Debug)]
pub struct Axon<'a, T> {
    /// Axon weight.
    pub weight: T,

    /// Incoming synapse.
    pub(crate) incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    pub(crate) outgoing: &'a Neuron<'a, T>,

    /// Bias.
    bias: CellKind<T>,
}

impl<'a> Axon<'a, f64> {
    pub(crate) fn new(inn: &'a Neuron<f64>, out: &'a Neuron<f64>) -> Self {
        Self {
            weight: 0.,
            incoming: inn,
            outgoing: out,
            bias: CellKind::Bias(true),
        }
    }
}
