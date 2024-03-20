use crate::activation::{Activation, get_derivative};
use crate::cell::{Axon, CellTrait, CoreTrait};

struct HiddenCell {
    /// Neuron value.
    value: f32,

    /// Neuron error.
    miss: f32,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,

    /// All outgoing axons.
    outgoing_axons: Vec<Axon>,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    _rate: f32,
}

impl HiddenCell {
    fn activation(&mut self) {}
    fn derivative(&mut self) {}

    fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        for axon in &mut self.outgoing_axons {
            axon.calculate_miss();
        }
    }

    fn update_weight(&mut self) {
        let gradient = self._rate
            * self.miss
            * get_derivative(&mut (self.value as f64), &self.activation_mode);

        for axon in &mut self.incoming_axons {
            axon.update_weight(&gradient);
        }
    }
}

impl CoreTrait for HiddenCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl CellTrait for HiddenCell {
    fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    fn set_miss(&mut self, value: f32) {
        self.miss = value;
    }

    fn get_incoming_axons(&self) -> &Vec<Axon> {
        &self.incoming_axons
    }
}
