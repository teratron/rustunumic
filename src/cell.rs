use crate::activation::{Activation, get_derivative};

mod bias;
mod input;
mod output;
mod hidden;

trait CoreTrait {
    fn get_value(&self) -> &f32;
}

trait CellTrait: CoreTrait {
    fn get_miss(&self) -> &f32;
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_weight(&mut self);
}

//************************************************************************

struct Axon {
    /// Axon weight.
    weight: f32,

    /// Incoming cell (InputCell, HiddenCell, BiasCell).
    incoming_cell: dyn CoreTrait,

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

//************************************************************************

trait Synapse {}
impl Synapse for Vec<Axon> {}
impl Synapse for (Vec<Axon>, Vec<Axon>) {}

//************************************************************************

struct CoreCell {
    /// Neuron value.
    value: f32,

    /// Neuron error.
    miss: f32,

    /// Function activation mode.
    activation_mode: Activation, //Option<Activation>,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,

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
