mod bias;
mod input;
mod output;
mod hidden;

use crate::activation::{get_derivative, Activation};

trait CoreTrait {
    fn get_value(&self) -> &f32;
}

trait CellTrait: CoreTrait {
    //fn set_value(&mut self, value: f32);
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
    fn get_miss(&self) -> &f32;
    //fn set_miss(&mut self, value: f32);
    /*fn calculate_miss(&mut self) {
        calculate_value(&mut self);
    }*/
    //fn get_incoming_axons(&self) -> Vec<Axon>;
}

//************************************************************************

struct Axon {
    /// Axon weight.
    weight: f32,

    // InputCell, HiddenCell, BiasCell
    /// Incoming cell.
    incoming_cell: dyn CoreTrait,

    // HiddenCell, OutputCell
    /// Outgoing cell.
    outgoing_cell: dyn CellTrait,
}

impl Axon {
    // Forward propagation.
    fn calculate_value(&self) -> f32 {
        /*self.outgoing_cell.value += self.incoming_cell.get_value() * self.weight;
        self.outgoing_cell.set_value(
            self.outgoing_cell.get_value() + self.incoming_cell.get_value() * self.weight
        );*/
        self.incoming_cell.get_value() * self.weight
    }

    // Backward propagation.
    fn calculate_miss(&self) -> f32 {
        /*self.incoming_cell.miss += self.outgoing_cell.get_miss() * self.weight;
        self.incoming_cell.set_miss(
            self.incoming_cell.get_miss() + self.outgoing_cell.get_miss() * self.weight
        );*/
        self.outgoing_cell.get_miss() * self.weight
    }

    fn update_weight(&mut self, gradient: &f32) {
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
    fn activation(&mut self) {}
    fn derivative(&mut self) {}

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn update_weight(&mut self) {
        let gradient = self._rate
            * self.miss
            * get_derivative(&mut (self.value as f64), &self.activation_mode);

        for axon in &mut self.incoming_axons {
            axon.update_weight(&gradient);
        }
    }
}

/*impl CoreTrait for CoreCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl CellTrait for CoreCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    fn set_miss(&mut self, value: f32) {
        self.miss = value;
    }
}*/

//************************************************************************

//************************************************************************

/*fn calculate_value(cell: &mut dyn CellTrait) {
    cell.set_value(0.);
    for axon in cell.get_incoming_axons() {
        cell.set_value(cell.get_value() + axon.calculate_value());
    }
}*/
