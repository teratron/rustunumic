//! # Axon
//!
//!

extern crate rand;

use rand::{thread_rng, Rng};

use super::cell::{Neuron, Nucleus};
use super::float::Float;

pub(super) struct Axon<T: Float> {
    /// Axon weight.
    weight: T,

    /// Incoming cell: InputCell, BiasCell, CoreCell.
    incoming_cell: Box<dyn Nucleus<T>>,

    /// Outgoing cell: CoreCell, OutputCell.
    outgoing_cell: Box<dyn Neuron<T>>,
}

impl<T: Float> Axon<T> {
    fn new() -> Box<Self> {
        let mut rng = thread_rng();
        Box::new(Self {
            weight: T::from(rng.gen_range(-0.5..=0.5)),
            incoming_cell: Box::new(todo!()),
            outgoing_cell: Box::new(todo!()),
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
