//! # Input Cell
//!
//!

use crate::cell::Nucleus;

struct InputCell<T>(T);

impl<T> Nucleus<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}
