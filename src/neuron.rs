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

/*
func (p *perceptron) calcNeuron(input []float64) {
    p.initSynapseInput(input)
    wait := make(chan bool)
    defer close(wait)
    for _, v := range p.neuron {
        for _, w := range v {
            go func(n *neuron) {
                n.value = 0
                for _, a := range n.axon {
                    n.value += a.getSynapseInput() * a.weight
                }
                n.value = floatType(calcActivation(float64(n.value), p.Conf.ActivationMode))
                wait <- true
            }(w)
        }
        for range v {
            <-wait
        }
    }
}
*/
