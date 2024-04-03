//! # Bias Cell
//!
//!

use crate::FloatTrait;

use super::NeuronBaseTrait;

struct BiasCell;

impl<T: FloatTrait> NeuronBaseTrait<T> for BiasCell {
    fn get_value(&self) -> &T {
        &T::ONE
    }
}
