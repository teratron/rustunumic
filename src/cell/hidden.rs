use crate::axon::Axon;
use crate::cell::{CellTrait, CoreCell, CoreTrait};

struct HiddenCell {
    /// Core cell.
    cell: CoreCell,

    /// All outgoing axons.
    outgoing_axons: Vec<Axon>,
}

impl HiddenCell {}

impl CoreTrait for HiddenCell {
    fn get_value(&self) -> &f32 {
        &self.cell.value
    }
}

impl CellTrait for HiddenCell {
    fn get_miss(&self) -> &f32 {
        &self.cell.miss
    }

    fn calculate_value(&mut self) {
        self.cell.calculate_value();
    }

    fn calculate_miss(&mut self) {
        self.cell.miss = 0.;
        for axon in &mut self.outgoing_axons {
            axon.calculate_miss();
        }
    }

    fn calculate_weight(&mut self) {
        self.cell.calculate_weight();
    }
}
