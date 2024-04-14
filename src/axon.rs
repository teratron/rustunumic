//! # Axon
//!
//!

extern crate rand;

use rand::{thread_rng, Rng};

use super::{Float, Neuron, NeuronBase};

pub(super) struct Axon<T> {
    /// Axon weight.
    weight: T,

    /// Incoming cell: InputCell, BiasCell, HiddenCell.
    incoming_cell: Box<dyn NeuronBase<T>>,

    /// Outgoing cell: HiddenCell, OutputCell.
    outgoing_cell: Box<dyn Neuron<T>>,
}

impl<T: Float> Axon<T> {
    fn new(incoming_cell: Box<dyn NeuronBase<T>>, outgoing_cell: Box<dyn Neuron<T>>) -> Box<Self> {
        let mut rng = thread_rng();
        Box::new(Self {
            weight: T::from(rng.gen_range(-0.5..=0.5)),
            incoming_cell,
            outgoing_cell,
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
