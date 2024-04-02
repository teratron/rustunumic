//! # Bias Cell
//!
//!

use crate::float::Float;

use super::NeuronBase;

struct BiasCell;

impl<T: Float> NeuronBase<T> for BiasCell {
    fn get_value(&self) -> &T {
        &T::ONE
    }
}
