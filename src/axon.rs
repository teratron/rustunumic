//! # Axon
//!
//!
extern crate rand;

use rand::{thread_rng, Rng};

use crate::neuron::Synapse;

//#[derive(Debug)]
pub(crate) struct Axon<'a, T> {
    /// Axon weight.
    pub weight: T,

    /// Incoming synapse.
    pub(crate) incoming: &'a dyn Synapse<T>,

    /// Outgoing synapse.
    pub(crate) outgoing: dyn Synapse<T>,

    //pub(crate) synapse: (&'a dyn Synapse<'a, T>, &'a dyn Synapse<'a, T>),
}

impl<T> Axon<'_, T> {
    pub(crate) fn calculate_forward_value(&self) -> T {
        todo!()
    }
}

impl<'a> Axon<'a, f32> {
    pub(crate) fn new(inn: impl Synapse<f32>, out: impl Synapse<f32>) -> &'a Self {
        let mut rng = thread_rng();

        &Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            //synapse: Synapse::new(inn, out),
        }
    }

    /*pub(crate) fn calculate_forward_value(&self) -> f32 {
        self.outgoing.get_value() + self.incoming.get_value() * self.weight
    }

    pub(crate) fn calculate_backward_value(&mut self) -> f32 {
        self.outgoing.get_value() + self.incoming.get_value() * self.weight
    }*/
}

impl<'a> Axon<'a, f64> {
    //pub(crate) fn new(inn: &'a dyn Synapse<'a, f64>, out: &'a dyn Synapse<'a, f64>) -> Self {
    pub(crate) fn new(inn: impl Synapse<f64>, out: impl Synapse<f64>) -> &'a Self {
        let mut rng = thread_rng();

        &Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            //synapse: Synapse::new(inn, out),
        }
    }

    pub(crate) fn back(&mut self) -> f64 {
        self.incoming.get_value() * self.weight
    }

    /*pub(crate) fn calculate_forward_value(&self) -> f64 {
        self.outgoing.get_value() + self.incoming.get_value() * self.weight
    }

    pub(crate) fn calculate_backward_value(&mut self) -> f64 {
        self.outgoing.get_value() + self.incoming.get_value() * self.weight
    }*/
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

mod tests {
    use super::*;

    #[test]
    fn test_new_axon() {
        let in_neuron = Synapse::new();
        let out_neuron = Synapse::new();

        let axon = Axon::new(&in_neuron, &out_neuron);

        assert_eq!(axon.incoming, &in_neuron);
        assert_eq!(axon.outgoing, &out_neuron);
    }

    #[test]
    fn test_random_weight() {
        let axon = Axon::new(&Synapse::new(), &Synapse::new());

        assert!(axon.weight >= -0.5);
        assert!(axon.weight <= 0.5);
    }

    #[test]
    fn test_synapse() {
        let in_neuron = Synapse::new();
        let out_neuron = Synapse::new();

        let axon = Axon::new(&in_neuron, &out_neuron);

        assert_eq!(axon.synapse.incoming, &in_neuron);
        assert_eq!(axon.synapse.outgoing, &out_neuron);
    }
}
