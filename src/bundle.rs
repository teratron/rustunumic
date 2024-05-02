//! # Bundle
//!
//!

//use std::fmt::Debug;

use crate::cell::Nucleus;

use super::{Float, HiddenCell, InputCell, Neuron, OutputCell};

//#[derive(Debug)]
pub(super) struct Bundle<T, S> {
    /// Reference to a slice of neurons.
    pub(super) cells: Box<[S]>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

// Common Bundle.
impl<T: Float, S> Bundle<T, S> {
    pub(super) fn new(number: usize) -> Self {
        Self {
            cells: Box::new([]), // TODO: ?
            ..Self::new_with_number(number)
        }
    }

    pub(super) fn new_with_number(number: usize) -> Self {
        let number_float = T::from(number as f64);
        Self {
            cells: Box::new([]),
            number,
            number_float,
        }
    }

    pub(super) fn set_number(&mut self, number: usize) {
        self.number = number;
        self.number_float = T::from(number as f64);
    }
}

// Bundle for OutputCell or HiddenCell.
impl<T, S> Bundle<T, S>
where
    T: Float,
    S: Neuron<T>,
{
    pub(super) fn get_values(&self) -> Vec<&T> {
        self.cells
            .iter()
            .map(|n| n.get_value())
            .collect::<Vec<&T>>()
    }

    pub(super) fn get_misses(&self) -> Vec<&T> {
        self.cells.iter().map(|n| n.get_miss()).collect::<Vec<&T>>()
    }
}

// Input Bundle.
impl<'a, T: Float> Bundle<T, InputCell<'a, T>> {
    // Помещает входные данные в сеть.
    pub(super) fn set_inputs(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_value(&v));
    }
}

// Output Bundle.
impl<'a, T: Float> Bundle<T, OutputCell<'a, T>> {
    // Помещает целевые данные в сеть.
    pub(super) fn set_targets(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_target(&v));
    }
}

// Hidden Bundle.
impl<T: Float> Bundle<T, HiddenCell<T>> {}
