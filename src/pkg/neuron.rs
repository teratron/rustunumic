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
    activation: Activation,
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
