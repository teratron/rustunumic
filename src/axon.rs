//! # Axon
//!
//!

#![allow(dead_code)]

extern crate rand;

use rand::{Rng, rng};

use super::{Float, Neuron, Nucleus};

pub(super) type AxonBundle<T> = Vec<Axon<T>>;

//#[derive(Debug)]
pub(super) struct Axon<T> {
    /// Axon weight.
    weight: T,

    /// Incoming cell: HiddenCell, InputCell, BiasCell.
    incoming_cell: Box<dyn Nucleus<T>>,

    /// Outgoing cell: HiddenCell, OutputCell.
    outgoing_cell: Box<dyn Neuron<T>>,
}

impl<T: Float> Axon<T> {
    pub(super) fn new(
        incoming_cell: Box<dyn Nucleus<T>>,
        outgoing_cell: Box<dyn Neuron<T>>,
    ) -> Self {
        let mut rng = rng();
        Self {
            weight: T::from_f64(rng.random_range(-0.5..=0.5)),
            incoming_cell,
            outgoing_cell,
        }
    }

    // -----------------------------------------------------------------------
    // Forward propagation.
    // -----------------------------------------------------------------------

    pub(super) fn calculate_value(&self) -> T {
        *self.incoming_cell.get_value() * self.weight
    }

    // -----------------------------------------------------------------------
    // Backward propagation.
    // -----------------------------------------------------------------------

    pub(super) fn calculate_miss(&self) -> T {
        *self.outgoing_cell.get_miss() * self.weight
    }

    pub(super) fn calculate_weight(&mut self, gradient: &T) {
        self.weight += *gradient * *self.incoming_cell.get_value();
    }
}
