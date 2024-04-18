//! # Query
//!
//!

use super::{Float, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Querying dataset.
    pub fn query(&mut self, input: &'a [T]) -> Vec<&T> {
        if !self.is_init {
            panic!("not initialized");
        }

        self.input_cells.set_inputs(input);
        self.calculate_values();
        self.is_query = true;
        self.output_cells.get_values()
    }
}
