//! # Network
//!
//!

use super::{Float, Neuron};
use super::{HiddenCell, InputCell, OutputCell};

pub(super) struct Network<T, S> {
    /// Reference to a slice of neurons.
    pub(super) neurons: Box<[S]>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

impl<T: Float, S: Neuron<T>> Network<T, S> {
    pub(super) fn new(number: usize) -> Self {
        Self {
            neurons: Box::new([]),
            number,
            number_float: T::from(number as f64), //self.get_number_float(),
        }
    }

    pub(super) fn set_number(&mut self, number: usize) {
        self.number = number;
        self.number_float = T::from(self.number as f64);
    }

    pub(super) fn get_collect_values(&self) -> Vec<&T> {
        self.neurons
            .iter()
            .map(|neuron| neuron.get_value())
            .collect::<Vec<&T>>()
    }

    pub(super) fn get_collect_misses(&self) -> Vec<&T> {
        self.neurons
            .iter()
            .map(|neuron| neuron.get_value())
            .collect::<Vec<&T>>()
    }

    /*pub fn get_number_float(&self) -> T {
        T::from(self.number as f64)
    }*/
}

struct Bundle<'a, T> {
    /// All working neurons.
    network: Vec<&'a dyn Neuron<T>>,

    /// Input neurons.
    pub(super) input_cells: Network<T, InputCell<T>>,

    /// Output neurons.
    pub(super) output_cells: Network<T, OutputCell<T>>,

    /// Hidden neurons.
    pub(super) hidden_cells: Network<T, HiddenCell<T>>,
}
