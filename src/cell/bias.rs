//! # Bias Cell
//!
//!

use crate::float::Float;

use super::Nucleus;

struct BiasCell;

impl<T: Float> Nucleus<T> for BiasCell {
    fn get_value(&self) -> &T {
        &T::ONE
    }
}
