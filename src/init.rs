//! # Initialization
//!
//!

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &[T], target: &[T]) -> bool {
        let number_input = input.len();
        let number_output = target.len();

        self.output_cells.set_number(number_output);

        //self.is_init = true;
        !self.is_init
    }
}
