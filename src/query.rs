//! # Query
//!
//!

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Querying dataset.
    pub fn query(&mut self, _input: &[T]) -> Vec<&T> {
        if !self.is_init {
            panic!("not initialized");
        }
        //self.input = input;
        self.calculate_values();
        self.is_query = true;
        self.output.get_collect_values()
    }
}
