//! # Initialization
//!
//!

#![allow(unused)]

use crate::cell::input::InputCell;

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &[T], target: &[T]) -> bool {
        let number_input = input.len();
        let number_output = target.len();

        /*let mut inp = Vec::with_capacity(number_input);
        input.iter().enumerate().for_each(|v| {
            inp[v.0] = InputCell::new(v.1);
        });*/

        let mut _input = input
            .iter()
            .map(|v| InputCell::new(v))
            .collect::<Vec<InputCell<T>>>();

        self.output_cells.set_number(number_output);

        //self.is_init = true;
        !self.is_init
    }
}
