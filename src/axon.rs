//! # Axon
//!
//!

extern crate rand;

use rand::{thread_rng, Rng};

use super::{Float, Neuron, NeuronBase};

pub(super) struct Axon<T> {
    /// Axon weight.
    weight: T,

    /// Incoming cell: InputCell, BiasCell, CoreCell.
    incoming_cell: Box<dyn NeuronBase<T>>,

    /// Outgoing cell: CoreCell, OutputCell.
    outgoing_cell: Box<dyn Neuron<T>>,
}

impl<T: Float> Axon<T> {
    fn new() -> Box<Self> {
        let mut rng = thread_rng();
        Box::new(Self {
            weight: T::from(rng.gen_range(-0.5..=0.5)),
            incoming_cell: Box::new((_)),
            outgoing_cell: Box::new((_)),
        })
    }

    // Forward propagation.

    pub(super) fn calculate_value(&self) -> T {
        *self.incoming_cell.get_value() * self.weight
    }

    // Backward propagation.

    pub(super) fn calculate_miss(&self) -> T {
        *self.outgoing_cell.get_miss() * self.weight
    }

    pub(super) fn calculate_weight(&mut self, gradient: &T) {
        self.weight += *gradient * *self.incoming_cell.get_value();
    }
}
