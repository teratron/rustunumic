//! # Input Cell
//!
//!

use super::NeuronBase;

pub(crate) struct InputCell<'a, T>(&'a T);

impl<T> InputCell<T> {
    fn new(value: &T) -> Self {
        InputCell(value)
    }
}

impl<T> NeuronBase<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}

struct InputData<'a, T> {
    neurons: &'a [T],
    cells: Vec<InputCell<T>>,
}
