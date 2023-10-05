use crate::activation::Activation;
use crate::axon::Axon;

#[derive(Debug)]
pub(crate) struct Neuron<'a, T> {
    /// Neuron value
    value: T,

    /// Neuron error
    miss: T,

    /// All incoming axons
    incoming: Vec<&'a Axon<'a, T>>,

    /// All outgoing axons
    outgoing: Vec<&'a Axon<'a, T>>,

    /// Function activation
    activation: Option<Activation>,
}

impl<'a> Neuron<'a, f64> {
    pub(crate) fn new() -> Self {
        Self {
            value: 0.,
            miss: 0.,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            activation: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NeuronInput<'a, T> {
    /// Neuron value
    value: T,

    /// All outgoing axons
    outgoing: Vec<&'a Axon<'a, T>>,
}

// #[derive(Debug)]
// pub(crate) struct NeuronOutput<'a, T> {
//     /// Neuron value
//     value: T,
//
//     /// Neuron error
//     miss: T,
//
//     /// All incoming axons
//     incoming: Vec<&'a Axon<'a, T>>,
//
//     /// All outgoing axons
//     outgoing: Vec<&'a Axon<'a, T>>,
//
//     /// Function activation
//     activation: Activation,
// }

#[derive(Debug)]
pub(crate) struct NeuronTarget<'a, T> {
    /// Neuron value
    value: T,

    /// All incoming axons
    incoming: Vec<&'a Axon<'a, T>>,
}

#[derive(Debug)]
pub(crate) enum CellKind<T> {
    Input(T),
    BackfedInput,
    NoisyInput,

    Hidden,
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias(bool),

    Output(T),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}
