use crate::activation::Activation;

#[derive(Debug)]
pub(super) struct Neuron<T> {
    /// Neuron value
    value: T,
    /// Neuron error
    miss: T,
    /// All incoming axons
    //axon: Vec<T>,
    /// Specific option of neuron: miss (error) or other
    //specific: Neuroner,

    //Synapser

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
    Bias,

    Output,
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}
