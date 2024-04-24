//! # Query
//!
//!

use super::{Float, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Querying dataset.
    ///
    /// # Arguments
    ///
    /// * `input` - Input dataset.
    ///
    /// # Returns
    ///
    /// * `Vec<&T>` - Output dataset.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustunumic::Rustunumic;
    /// ```
    pub fn query(&mut self, input: &'a [T]) -> Vec<&T> {
        if !self.is_init {
            panic!("not initialized");
        }

        self.network.input.set_inputs(input);
        self.calculate_values();
        self.is_query = true;
        self.network.output.get_values()
    }
}
