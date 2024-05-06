//! # Initialization
//!
//!

#![allow(unused)]

use super::{Bundle, Float, HiddenCell, InputCell, OutputCell, Rustunumic};

impl<T: Float> Rustunumic<'_, T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &[T], target: &[T]) -> bool {
        //let number_input = input.len();
        //let number_output = target.len();

        self.network.input = Bundle::<T, InputCell<T>>::new(input);
        self.network.output = Bundle::<T, OutputCell<T>>::new(target);

        if self.hidden_layers.is_empty() {
            self.network.hidden = Bundle::<T, HiddenCell<T>>::new(&[]);
        } else {
            self.network.hidden = Bundle::<T, HiddenCell<T>>::new(
                &self
                    .hidden_layers
                    .iter()
                    .map(|&number_neurons| T::ONE)
                    .collect::<Vec<T>>(),
            );
        }

        self.network.cells.clear();

        !self.is_init //self.is_init = true;
    }
}
