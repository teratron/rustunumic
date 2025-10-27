//! # Initialization
//!
//!

#![allow(unused)]

use super::{Bundle, Float, InputCell, Network, OutputCell, Rustunumic};
use tracing::{debug, error, info, trace, warn, Level};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &'a [T], target: &'a [T]) -> bool {
        debug!("Initializing network with {} inputs and {} targets", input.len(), target.len());
        //let number_input = input.len();
        //let number_output = target.len();

        self.network = Network::<T>::new();
        trace!("Created new network instance");

        self.network.input = Bundle::<T, InputCell<T>>::_new(input);
        self.network.output = Bundle::<T, OutputCell<T>>::_new(target);
        trace!("Input and output bundles created");

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
        trace!("Network cells cleared");

        let result = !self.is_init;
        self.is_init = true;
        info!("Network initialization completed");
        result //self.is_init = true;
    }
}
