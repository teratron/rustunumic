//! # Input Cell
//!
//!

use super::Nucleus;

struct InputCell<T>(T);

impl<T> Nucleus<T> for InputCell<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}
