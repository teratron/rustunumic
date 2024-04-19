//! # Network
//!
//!

#![allow(dead_code)]

use super::HiddenCell;
use super::InputBundle;
use super::OutputCell;
use super::{Float, Neuron};

pub(super) struct Network<'a, T> {
    /// All working neurons.
    pub(super) cells: Vec<&'a dyn Neuron<T>>, //Vec<Box<dyn Neuron<T>>>,

    /// Input neurons.
    pub(crate) input: InputBundle<'a, T>,

    /// Output neurons.
    pub(crate) output: Bundle<T, OutputCell<'a, T>>,

    /// Hidden neurons.
    pub(crate) hidden: Bundle<T, HiddenCell<T>>,
}

impl<'a, T> Network<'a, T> {
    pub(crate) fn new() -> Self {
        todo!()
    }
}

pub(super) struct Bundle<T, S> {
    /// Reference to a slice of neurons.
    pub(super) cells: Box<[S]>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

impl<T: Float, S: Neuron<T>> Bundle<T, S> {
    pub(super) fn new(number: usize) -> Self {
        Self {
            cells: Box::new([]),
            number,
            number_float: T::from(number as f64), //self.get_number_float(),
        }
    }

    pub(super) fn get_values(&self) -> Vec<&T> {
        self.cells
            .iter()
            .map(|n| n.get_value())
            .collect::<Vec<&T>>()
    }

    pub(super) fn get_misses(&self) -> Vec<&T> {
        self.cells.iter().map(|n| n.get_miss()).collect::<Vec<&T>>()
    }

    /*pub(super) fn set_number(&mut self, number: usize) {
        self.number = number;
        self.number_float = T::from(self.number as f64);
    }
    pub fn get_numr_float(&self) -> T {
        T::from(self.number as f64)
    }*/
}

impl<'a, T: Float> Bundle<T, OutputCell<'a, T>> {
    pub(super) fn set_targets(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_target(&v));
    }
}
