//! # Query
//!
//!

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Querying dataset.
    pub fn query(&mut self, input: &[T]) -> Vec<&T> {
        if !self.is_init {
            panic!("not initialized");
        }

        self.network.set_input_data(input);

        //self.input = input;
        //self.
        self.calculate_values();
        self.is_query = true;
        self.network.output_cells.get_collect_values()
    }
}
