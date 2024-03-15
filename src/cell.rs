struct Axon {
    weight: f32,
    incoming_cell: InCell,
    outgoing_cell: OutCell,
}

struct CoreCell {
    activation_function: fn(f32) -> f32,
    bias: bool,
}

struct InCell {
    value: f32,
    incoming_axons: Vec<Axon>,
}

struct OutCell {
    miss: f32,
    outgoing_axons: Vec<Axon>,
}

struct InputCell(f32);

struct TargetCell(f32);
