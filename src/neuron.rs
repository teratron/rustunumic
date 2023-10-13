//! # Neuron
//!
//!

use crate::activation::{activation, get_activation, Activation};
use crate::axon::Axon;

#[derive(Debug)]
pub(crate) struct Neuron<'a, T> {
    /// Neuron value.
    pub value: T,

    /// Neuron error.
    pub miss: T,

    /// All incoming axons.
    pub incoming: Box<Vec<&'a Axon<'a, T>>>,

    /// All outgoing axons.
    pub outgoing: Box<Vec<&'a Axon<'a, T>>>,

    /// Function activation.
    pub activation: Option<Activation>,
}

impl<'a> Neuron<'a, f64> {
    pub(crate) fn new() -> Self {
        Self {
            value: 0.,
            miss: 0.,
            incoming: Box::new(Vec::new()), //.push(Axon<'a, f64>::new()),
            outgoing: Box::new(Vec::new()),
            activation: None,
        }
    }

    pub(crate) fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming.iter_mut() {
            self.value += axon.incoming.value * axon.weight;
        }

        if let Some(mode) = &self.activation {
            //self.value = activation(&self.value, mode);
            get_activation(&mut self.value, mode);
        }
    }

    pub(crate) fn calculate_loss(&mut self) {

    }

    pub(crate) fn calculate_miss(&mut self) {

    }

    pub(crate) fn update_axons(&mut self) {

    }
}

/*#[derive(Debug)]
pub(crate) struct NeuronInput<'a, T> {
    /// Neuron value.
    value: T,

    /// All outgoing axons.
    outgoing: Vec<&'a Axon<'a, T>>,
}*/

/*#[derive(Debug)]
pub(crate) struct NeuronOutput<'a, T> {
    target: T,

    /// Neuron value
    value: T,

    /// Neuron error
    miss: T,

    /// All incoming axons
    incoming: Vec<&'a Axon<'a, T>>,

    /// All outgoing axons
    outgoing: Vec<&'a Axon<'a, T>>,

    /// Function activation
    activation: Activation,
}*/

/*#[derive(Debug)]
pub(crate) struct NeuronTarget<'a, T> {
    /// Neuron value.
    value: T,
}*/

type AxonsType<'a, T> = Box<Vec<&'a Axon<'a, T>>>;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CellKind<'a, T> {
    Input(T, AxonsType<'a, T>),
    BackfedInput,
    NoisyInput,

    Hidden,
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias(bool),

    Output(T, Neuron<'a, T>),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}
