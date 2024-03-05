//! # Axon
//!
//!
extern crate rand;

use rand::{thread_rng, Rng};
use crate::neuron::Neuron;

#[derive(Debug)]
pub struct Axon<'a, T> {
    /// Axon weight.
    pub weight: T,

    /// Incoming synapse.
    pub(crate) incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    pub(crate) outgoing: &'a Neuron<'a, T>,

    //pub(crate) synapse: Synapse<'a, T>,
}

/*impl<'a, T: Float> Axon<'a, T> {
    pub(crate) fn new(inn: &'a Neuron<T>, out: &'a Neuron<T>) -> Self {
        //let mut rng = thread_rng();

        Self {
            weight: generate_random_weight::<T>(), //rng.gen_range(-0.5..=0.5), //T::ZERO,
            incoming: inn,
            outgoing: out,
            synapse: Synapse::new(inn, out)
        }
    }
}*/

impl<'a> Axon<'a, f32> {
    pub(crate) fn new(inn: &'a Neuron<f32>, out: &'a Neuron<f32>) -> Self {
        let mut rng = thread_rng();

        Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            //synapse: Synapse::new(inn, out),
        }
    }

    pub(crate) fn back(&mut self) -> f32 {
        self.incoming.value * self.weight
    }
}

impl<'a> Axon<'a, f64> {
    pub(crate) fn new(inn: &'a Neuron<f64>, out: &'a Neuron<f64>) -> Self {
        let mut rng = thread_rng();

        Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            //synapse: Synapse::new(inn, out),
        }
    }
    
    pub(crate) fn back(&mut self) -> f64 {
        self.incoming.value * self.weight
    }
}


#[derive(Debug)]
struct Synapse<'a, T> {
    /// Incoming synapse.
    incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    outgoing: &'a Neuron<'a, T>,
}

impl<'a, T> Synapse<'a, T> {
    fn new(p0: &'a Neuron<T>, p1: &'a Neuron<T>) -> Self {
        Self {
            incoming: p0, //Neuron::new(),
            outgoing: p1, //Neuron::new()
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_axon() {
        let in_neuron = Neuron::new();
        let out_neuron = Neuron::new();

        let axon = Axon::new(&in_neuron, &out_neuron);

        assert_eq!(axon.incoming, &in_neuron);
        assert_eq!(axon.outgoing, &out_neuron);
    }

    #[test]
    fn test_random_weight() {
        let axon = Axon::new(&Neuron::new(), &Neuron::new());

        assert!(axon.weight >= -0.5);
        assert!(axon.weight <= 0.5);
    }

    #[test]
    fn test_synapse() {
        let in_neuron = Neuron::new();
        let out_neuron = Neuron::new();

        let axon = Axon::new(&in_neuron, &out_neuron);

        assert_eq!(axon.synapse.incoming, &in_neuron);
        assert_eq!(axon.synapse.outgoing, &out_neuron);
    }
}
