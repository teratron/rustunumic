use crate::neuron::{CellKind, Neuron};

#[derive(Debug)]
pub struct Axon<'a, T> {
    /// Axon weight.
    weight: T,

    /// Incoming synapse.
    incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    outgoing: &'a Neuron<'a, T>,

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
