use crate::cell::{CellTrait, CoreCell, CoreTrait};

struct OutputCell {
    /// Core cell.
    cell: CoreCell,

    /// Target neuron.
    target: f32,
}

impl OutputCell {}

impl CoreTrait for OutputCell {
    fn get_value(&self) -> &f32 {
        &self.cell.value
    }
}

impl CellTrait for OutputCell {
    fn calculate_value(&mut self) {
        self.cell.calculate_value();
    }

    fn calculate_miss(&mut self) {
        self.cell.miss = self.target - self.cell.value;
    }

    fn calculate_weight(&mut self) {
        self.cell.calculate_weight();
    }
    
    fn get_miss(&self) -> &f32 {
        &self.cell.miss
    }
}
