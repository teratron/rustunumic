//! # Input Cell
//!
//!

use super::NeuronBase;

struct InputCell<T>(T);

impl<T> NeuronBase<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}
