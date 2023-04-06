// trait Neuroner {}
//
// impl<T> Neuroner for Neuron<T> {}
//
// struct Neuron<T> {
//     value: T, // Neuron value
//     axon: [T], // All incoming axons
//               //specific: Neuroner,  // Specific option of neuron: miss (error) or other
// }

// Neuroner
// type Neuroner interface {
// pkg.GetSetter
// }
//
// // neuron
// type neuron struct {
//     value    floatType // Neuron value
//     axon     []*axon   // All incoming axons
//     specific Neuroner  // Specific option of neuron: miss (error) or other
//
//     Synapser
// }

enum CellKind {
    Input,
    BackfedInput,
    NoisyInput,

    Hidden,
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,

    Output,
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}
