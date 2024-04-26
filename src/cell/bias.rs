//! # Bias Cell
//!
//!

use std::fmt::{Debug, Formatter, Result};

use crate::Float;

use super::Nucleus;

struct BiasCell<T>(T);

impl<T: Float> BiasCell<T> {
    pub(super) fn new() -> Self {
        BiasCell(T::ONE)
    }
}

impl<T> Nucleus<T> for BiasCell<T>
where
    T: Debug,
{
    fn get_value(&self) -> &T {
        &self.0
    }
}

// Debugging.
impl<T> Debug for BiasCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("BiasCell").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bias = BiasCell::<f32>::new();
        assert_eq!(bias.0, 1.);
    }

    #[test]
    fn test_get_value() {
        let bias = BiasCell::<f64>::new();
        assert_eq!(bias.get_value(), &1.);
    }
}
