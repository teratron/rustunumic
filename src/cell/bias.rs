//! # Bias Cell
//!
//!

use super::Nucleus;

struct BiasCell;

impl<T> Nucleus<T> for BiasCell {
    fn get_value(&self) -> &T {
        &1.
    }
}
