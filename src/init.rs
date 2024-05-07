//! # Initialization
//!
//!

#![allow(unused)]

use super::{Bundle, Float, InputCell, Network, OutputCell, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &'a [T], target: &'a [T]) -> bool {
        //let number_input = input.len();
        //let number_output = target.len();

        self.network = Network::<T>::new();

        self.network.input = Bundle::<T, InputCell<T>>::_new(input);
        self.network.output = Bundle::<T, OutputCell<T>>::_new(target);

        /*if self.hidden_layers.is_empty() {
            self.network.hidden = Bundle::<T, HiddenCell<T>>::_new(&[]);
        } else {
            self.network.hidden = Bundle::<T, HiddenCell<T>>::_new(
                &self
                    .hidden_layers
                    .iter()
                    .map(|&number_neurons| T::ONE)
                    .collect::<Vec<T>>(),
            );
        }*/

        self.network.cells.clear();

        !self.is_init //self.is_init = true;
    }
}
