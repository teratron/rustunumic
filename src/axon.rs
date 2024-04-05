//! # Axon
//!
//!

extern crate rand;

use rand::{Rng, thread_rng};

use super::{Float, Neuron, NeuronBase};

pub(super) struct Axon<T> {
    /// Axon weight.
    weight: T,

    /// Incoming cell: InputCell, BiasCell, CoreCell.
    incoming: Box<dyn NeuronBase<T>>,

    /// Outgoing cell: CoreCell, OutputCell.
    outgoing: Box<dyn Neuron<T>>,
}

impl<T: Float> Axon<T> {
    fn new() -> Box<Self> {
        let mut rng = thread_rng();
        Box::new(Self {
            weight: T::from(rng.gen_range(-0.5..=0.5)),
            incoming: Box::new((_)),
            outgoing: Box::new((_)),
        })
    }

    // Forward propagation.

    pub(super) fn calculate_value(&self) -> T {
        *self.incoming.get_value() * self.weight
    }

    // Backward propagation.

    pub(super) fn calculate_miss(&self) -> T {
        *self.outgoing.get_miss() * self.weight
    }

    pub(super) fn calculate_weight(&mut self, gradient: &T) {
        self.weight += *gradient * *self.incoming.get_value();
    }
}
