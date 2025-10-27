//! # Query
//!
//!

use super::{Float, Rustunumic};
use tracing::{debug, error, trace, Level};

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
        trace!("Starting query with {} input values", input.len());
        if !self.is_init {
            error!("Network not initialized before query");
            panic!("not initialized");
        }

        self.network.input.set_inputs(input);
        trace!("Input values set, starting value calculation");
        self.calculate_values();
        trace!("Value calculation completed");
        self.is_query = true;
        let output = self.network.output.get_values();
        debug!("Query completed, returning {} output values", output.len());
        output
    }
}
