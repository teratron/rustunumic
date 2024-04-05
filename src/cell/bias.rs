//! # Bias Cell
//!
//!

use super::{Float, NeuronBase};

struct BiasCell;

impl<T: Float> NeuronBase<T> for BiasCell {
    fn get_value(&self) -> &T {
        &T::ONE
    }
}
