use crate::activation::Activation;
use crate::axon::Axon;

#[derive(Debug)]
pub(super) struct Neuron<'a, T> {
    /// Neuron value
    value: T,

    /// Neuron error
    miss: T,

    /// All incoming axons
    incoming: Vec<Axon<'a, T>>,

    /// All outcoming axons
    outcoming: Vec<Axon<'a, T>>,

    /// Specific option of neuron: miss (error) or other
    //specific: Neuroner,
    //Synapser

    /// Function activation
    activation: Activation,
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
