//! # Bundle
//!
//!

use super::{Float, HiddenCell, InputCell, Neuron, OutputCell};

//#[derive(Debug)]
pub(super) struct Bundle<T, S> {
    /// Reference to a slice of neurons.
    pub(super) cells: Box<Vec<S>>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

// Common Bundle.
impl<T, S> Bundle<T, S>
where
    T: Float,
{
    pub(super) fn new() -> Self {
        println!("Bundle::new");
        Self {
            cells: Box::new(Vec::new()),
            number: 0,
            number_float: T::ZERO,
        }
    }

    pub(super) fn new_with(data: &[T]) -> Self {
        let number = data.len();
        Self {
            cells: Box::new(Vec::new()),
            number,
            number_float: T::from(number as f64),
        }
    }

    /*pub(super) fn set_number(&mut self, number: usize) {
        self.number = number;
        self.number_float = T::from(number as f64);
    }*/
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
    pub(super) fn _new(data: &'a [T]) -> Self {
        Self {
            cells: Box::new(if data.is_empty() {
                Vec::new()
            } else {
                data.iter().map(|v| InputCell::new(v)).collect()
            }),
            ..Self::new_with(data)
        }
    }

    // Помещает входные данные в сеть.
    pub(super) fn set_inputs(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_value(v));
    }
}

// Output Bundle.
impl<'a, T: Float> Bundle<T, OutputCell<'a, T>> {
    pub(super) fn _new(data: &'a [T]) -> Self {
        Self {
            cells: Box::new(data.iter().map(|v| OutputCell::new(v)).collect()),
            ..Self::new_with(data)
        }
    }

    // Помещает целевые данные в сеть.
    pub(super) fn set_targets(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_target(v));
    }
}

// Hidden Bundle.
impl<T: Float> Bundle<T, HiddenCell<T>> {
    pub(super) fn _new(data: &[T]) -> Self {
        Self {
            cells: Box::new(data.iter().map(|_| HiddenCell::new()).collect()),
            ..Self::new_with(data)
        }
    }
}

impl<T: Float, S> Default for Bundle<T, S> {
    fn default() -> Self {
        Self {
            cells: Box::new(Vec::new()),
            number: 0,
            number_float: T::ZERO,
        }
    }
}
