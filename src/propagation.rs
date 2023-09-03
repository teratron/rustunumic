impl Neuron<T> {
    fn calc_neurons(&self) -> Option<T> {
        //for i in &self.
        Option(_)
    }
}

fn calc_neuron() -> Option {}

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
