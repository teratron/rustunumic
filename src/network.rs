//! # Network
//!
//!

#![allow(dead_code)]

use crate::cell::output::OutputCell;

use super::{Float, Neuron};

/*pub(super) struct Network<'a, T> {
    /// All working neurons.
    pub(super) neurons: Vec<&'a dyn Neuron<T>>, //Vec<Box<dyn Neuron<T>>>,

    /// Input neurons.
    pub(super) input_cells: Bundle<T, InputCell<'a, T>>,

    /// Output neurons.
    pub(super) output_cells: Bundle<T, OutputCell<T>>,

    /// Hidden neurons.
    pub(super) hidden_cells: Bundle<T, HiddenCell<T>>,
}*/

pub(super) struct Bundle<T, S> {
    /// Reference to a slice of neurons.
    //pub(super) neurons: Box<[S]>,
    pub(super) neurons: Box<[S]>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

impl<T: Float, S: Neuron<T>> Bundle<T, S> {
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
            .map(|n| n.get_value())
            .collect::<Vec<&T>>()
    }

    /*pub(super) fn get_collect_misses(&self) -> Vec<&T> {
        self.neurons
            .iter()
            .map(|n| n.get_miss())
            .collect::<Vec<&T>>()
    }*/

    /*pub fn get_number_float(&self) -> T {
        T::from(self.number as f64)
    }*/
}

impl<'a, T: Float> Bundle<T, OutputCell<'a, T>> {
    pub(super) fn set_target_data(&mut self, target_data: &'a [T]) {
        target_data
            .iter()
            .enumerate()
            .for_each(|v| self.neurons[v.0].set_target(&v.1));
    }
}
