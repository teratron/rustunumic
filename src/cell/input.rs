//! # Input Cell
//!
//!

use super::NeuronBaseTrait;

struct InputCell<T>(T);

impl<T> NeuronBaseTrait<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}
