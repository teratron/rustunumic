use crate::activation::{get_derivative, Activation};

trait CoreTrait {
    fn get_value(&self) -> &f32;
}

trait CellTrait {
    fn get_miss(&self) -> &f32;
}

struct Axon {
    weight: f32,

    // InCell, InputCell, BiasCell
    incoming_cell: dyn CoreTrait,

    // OutCell, TargetCell
    outgoing_cell: OutCell,
}

impl Axon {
    // Forward propagation.
    fn calculate_value(&mut self) {
        self.outgoing_cell.value += self.incoming_cell.get_value() * self.weight;
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.incoming_cell.miss += self.outgoing_cell.get_miss() * self.weight;
    }

    fn update_weight(&mut self, gradient: f32) {
        self.weight += gradient * self.incoming_cell.get_value();
    }
}

//************************************************************************

struct CoreCell {
    activation_function: Option<Activation>, //fn(f32) -> f32,
}

//************************************************************************

struct InCell {
    value: f32,
    miss: f32,
    incoming_axons: Vec<Axon>,

    rate: f32,
    activation_function: Activation,
    cell: OutCell,
}

impl InCell {
    fn activation(&mut self) {}
    fn derivative(&mut self) {}

    fn get_value(&self) -> &f32 {
        &self.value
    }

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in &mut self.incoming_axons {
            axon.calculate_value();
        }
    }

    // Backward propagation.
    fn update_weight(&mut self) {
        let gradient = self.rate
            * self.cell.miss
            * get_derivative(&mut (self.value as f64), &self.activation_function);

        for axon in &mut self.incoming_axons {
            axon.update_weight(gradient);
        }
    }
}

//************************************************************************

struct OutCell {
    value: f32,
    miss: f32,
    outgoing_axons: Vec<Axon>,
}

impl OutCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    // Backward propagation.
    fn calculate_miss(&mut self) {
        self.miss = 0.;
        for axon in &mut self.outgoing_axons {
            axon.calculate_miss();
        }
    }
}

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
}

//************************************************************************

struct OutputCell {
    target: f32
}

//************************************************************************

struct TargetCell(f32);
