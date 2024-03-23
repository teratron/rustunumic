//! # Axon
//!
//!

extern crate rand;

use rand::{Rng, thread_rng};

use crate::cell::{Neuron, Nucleus};

pub(super) struct Axon {
    /// Axon weight.
    weight: f32,

    /// Incoming cell (InputCell, BiasCell, HiddenCell).
    incoming_cell: Box<dyn Nucleus>,

    /// Outgoing cell (HiddenCell, OutputCell).
    outgoing_cell: Box<dyn Neuron>,
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

pub(super) trait Synapse {
    fn get_incoming_axons(&self) -> &Self;
    fn get_outgoing_axons(&mut self) -> &Self;
}

impl Synapse for Vec<Axon> {
    fn get_incoming_axons(&self) -> &Self {
        self
    }

    fn get_outgoing_axons(&mut self) -> &Self {
        self
    }
}

impl Synapse for (Vec<Axon>, Vec<Axon>) {
    fn get_incoming_axons(&self) -> &Self {
        self
    }

    fn get_outgoing_axons(&mut self) -> &Self {
        self
    }
}

/*
pub(super) enum Synapse {
    Incoming(Vec<Axon>),
    Outgoing(Vec<Axon>),
}

struct Synapse {
    incoming_axons: Vec<Axon>,
    outgoing_axons: Option<Vec<Axon>>,
}
*/
