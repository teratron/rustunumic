// trait Neuroner {}
//
// impl<T> Neuroner for Neuron<T> {}
//

struct Neuron<T> {
    // Neuron value
    value: T,

    // All incoming axons
    axon: [T],
    // Specific option of neuron: miss (error) or other
    //specific: Neuroner,

    //Synapser
}

pub(crate) enum CellKind {
    Input,
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
