//! # Input Cell
//!
//!

use super::NeuronBase;

pub(crate) struct InputCell<T>(T);

impl<T> InputCell<T> {
    fn new(value: T) -> Self {
        InputCell(value)
    }
}

impl<T> NeuronBase<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}
