use crate::activation::{get_derivative, Activation};

trait CoreTrait {
    fn get_value(&self) -> &f32;
    //fn set_value(&mut self, value: f32);
}

trait CellTrait: CoreTrait {
    fn get_miss(&self) -> &f32;
    //fn set_miss(&mut self, value: f32);
}

trait KindTrait {
    fn calculate_miss(&mut self);
}

//trait IncomingSynapse: CoreTrait {}

//trait OutgoingSynapse: CoreTrait {}

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

struct BiasCell;

impl CoreTrait for BiasCell {
    fn get_value(&self) -> &f32 {
        &1.
    }
}

//************************************************************************

struct InputCell(f32);

impl CoreTrait for InputCell {
    fn get_value(&self) -> &f32 {
        &self.0
    }

    /*fn set_value(&mut self, value: f32) {
        self.0 = value;
    }*/
}

//************************************************************************

struct CoreCell {
    /// Neuron value.
    value: f32,

    /// Neuron error.
    miss: f32,

    /// Function activation mode.
    activation_mode: Option<Activation>,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,
    //&'a
    _rate: f32,

    // incoming_axons or (incoming_axons, outgoing_axons)
    // Vec<Axon> or (Vec<Axon>, Vec<Axon>)
    // (Vec<Axon>, Option<Vec<Axon>>)
    synapses: dyn Synapse,

    // HiddenCell, OutputCell
    cell: dyn KindTrait,
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
            * get_derivative(&mut (self.value as f64), &self.activation_mode.unwrap());

        for axon in &mut self.incoming_axons {
            axon.update_weight(&gradient);
        }
    }
}

impl CoreTrait for CoreCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl CellTrait for CoreCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    /*fn set_miss(&mut self, value: f32) {
        self.miss = value;
    }*/
}

//************************************************************************

struct OutputCell {
    /// Target neuron.
    target: f32,
}

impl KindTrait for OutputCell {
    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = self.target - self.value;
    }
}

//************************************************************************

struct HiddenCell {
    /// All outgoing axons.
    outgoing_axons: Vec<Axon>,
}

impl KindTrait for HiddenCell {
    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        for axon in &mut self.outgoing_axons {
            axon.calculate_miss();
        }
    }
}
