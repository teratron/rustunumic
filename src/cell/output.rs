//use crate::activation::{Activation, get_derivative};
use crate::cell::{Axon, CellTrait, CoreCell, CoreTrait};

struct OutputCell {
    cell: CoreCell,
    /// Neuron value.
    //value: f32,

    /// Neuron error.
    //miss: f32,

    /// Target neuron.
    target: f32,

    /// All incoming axons.
    //incoming_axons: Vec<Axon>,

    /// Function activation mode.
    //activation_mode: Activation,

    //_rate: f32,
}

impl OutputCell {
    /*fn set_value(&mut self, value: f32) {
        self.value = value;
    }*/

    // Forward propagation.
    /*fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }*/

    // Backward propagation.
    /*fn calculate_miss(&mut self) {
        self.cell.miss = self.target - self.cell.value;
    }*/

    /*fn update_weight(&mut self) {
        let gradient = self._rate
            * self.miss
            * get_derivative(&mut (self.value as f64), &self.activation_mode);

        for axon in &mut self.incoming_axons {
            axon.update_weight(gradient);
        }
    }*/
}

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

    fn get_miss(&self) -> &f32 {
        &self.cell.miss
    }
}
