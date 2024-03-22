//! # Hidden Cell
//!
//!

use crate::cell::{CellTrait, CoreTrait};
//use crate::axon::Axon;
use crate::cell::core::CoreCell;

struct HiddenCell {
    /// Core cell.
    core: CoreCell,
    //// All outgoing axons.
    //outgoing_axons: Vec<Axon>,
}

impl HiddenCell {}

impl CoreTrait for HiddenCell {
    fn get_value(&self) -> &f32 {
        &self.core.value
    }
}

impl CellTrait for HiddenCell {
    fn get_miss(&self) -> &f32 {
        &self.core.miss
    }

    fn calculate_value(&mut self) {
        self.core.calculate_value();
    }

    fn calculate_miss(&mut self) {
        self.core.miss = 0.;
        //for axon in &mut self.outgoing_axons {
        for axon in self.core.synapses.1.as_mut().unwrap() {
            axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self) {
        self.core.calculate_weight();
    }
}
