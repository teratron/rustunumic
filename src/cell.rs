use crate::activation::Activation;

struct Axon {
    weight: f32,
    incoming_cell: InCell,
    // InCell, InputCell, BiasCell
    outgoing_cell: OutCell,
    // OutCell, TargetCell
}

impl Axon {
    fn calculate_value(&self, cell: &mut InCell) {
        cell.value += self.incoming_cell.value * self.weight;
    }
}

//************************************************************************

struct CoreCell {
    activation_function: Option<Activation>, //fn(f32) -> f32,
}

//************************************************************************

struct InCell {
    value: f32,
    incoming_axons: Vec<Axon>,
}

impl InCell {
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in &self.incoming_axons {
            //self.value += axon.calculate_forward_value();
            axon.calculate_value(self);
        }
    }
}

//************************************************************************

struct OutCell {
    miss: f32,
    outgoing_axons: Vec<Axon>,
}

impl OutCell {
    fn calculate_miss(&mut self) {
        for axon in &self.incoming_axons {
            axon.calculate_value(self);
        }
    }
}

//************************************************************************

struct BiasCell(bool);

struct InputCell(f32);

struct TargetCell(f32);
