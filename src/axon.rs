//! # Axon
//!
//!

extern crate rand;

use rand::{thread_rng, Rng};

use crate::cell::{Neuron, Nucleus};

pub(super) struct Axon {
    /// Axon weight.
    weight: f32,

    /// Incoming cell (InputCell, BiasCell, CoreCell).
    incoming_cell: dyn Nucleus,

    /// Outgoing cell (CoreCell, OutputCell).
    outgoing_cell: dyn Neuron,
}

impl Axon {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming_cell: Box::new(()),
            outgoing_cell: Box::new(()),
        }
    }

    // Forward propagation.
    pub(super) fn calculate_value(&self) -> f32 {
        self.incoming_cell.get_value() * self.weight
    }

    // Backward propagation.
    pub(super) fn calculate_miss(&self) -> f32 {
        self.outgoing_cell.get_miss() * self.weight
    }

    pub(super) fn calculate_weight(&mut self, gradient: &f32) {
        self.weight += gradient * self.incoming_cell.get_value();
    }
}
