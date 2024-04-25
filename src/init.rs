//! # Initialization
//!
//!

#![allow(unused)]

use super::{Bundle, Float, Rustunumic};
use super::{InputCell, OutputCell};

impl<T: Float> Rustunumic<'_, T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &[T], target: &[T]) -> bool {
        let number_input = input.len();
        let number_output = target.len();

        let mut inp = Bundle::<T, InputCell<T>>::new(number_input);
        let mut out = Bundle::<T, OutputCell<T>>::new(number_output);

        self.network.cells.clear();

        !self.is_init //self.is_init = true;
    }
}
