use crate::activation::{get_derivative, Activation};
use crate::axon::Axon;

mod bias;
mod hidden;
mod input;
mod output;

pub(crate) trait CoreTrait {
    fn get_value(&self) -> &f32;
}

pub(crate) trait CellTrait: CoreTrait {
    fn get_miss(&self) -> &f32;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self);
}

/*fn calculate_value(cell: &mut dyn CellTrait) { // TODO: return value
    cell.calculate_value();
}*/

//************************************************************************

pub(crate) struct CoreCell {
    /// Neuron value.
    value: f32,

    /// Neuron error.
    miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,

    synapses: (Vec<Axon>, Option<Vec<Axon>>), //dyn Synapse,

    _rate: f32,
}

impl CoreCell {
    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn calculate_weight(&mut self) {
        let gradient = self._rate
            * self.miss
            * get_derivative(&mut (self.value as f64), &self.activation_mode);

        for axon in &mut self.incoming_axons {
            axon.calculate_weight(&gradient);
        }
    }
}
