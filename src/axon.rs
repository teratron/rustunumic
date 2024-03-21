//! # Axon
//!
//!

use crate::cell::{CellTrait, CoreTrait};

pub(crate) struct Axon {
    /// Axon weight.
    weight: f32,

    /// Incoming cell (InputCell, HiddenCell, BiasCell).
    incoming_cell: Box<dyn CoreTrait>,

    /// Outgoing cell (HiddenCell, OutputCell).
    outgoing_cell: dyn CellTrait,
}

impl Axon {
    // Forward propagation.
    fn calculate_value(&self) -> f32 {
        self.incoming_cell.get_value() * self.weight
    }

    // Backward propagation.
    fn calculate_miss(&self) -> f32 {
        self.outgoing_cell.get_miss() * self.weight
    }

    fn calculate_weight(&mut self, gradient: &f32) {
        self.weight += gradient * self.incoming_cell.get_value();
    }
}
