use super::Float;
use super::Neuron;
use super::{InputCell, OutputCell};

#[derive(Debug)]
pub(super) struct Bundle<T, S> {
    /// Reference to a slice of neurons.
    pub(super) cells: Box<[S]>,

    /// Number neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

impl<T: Float, S> Bundle<T, S> {
    pub(super) fn set_number(number: usize) -> Self {
        let number_float = T::from(number as f64);
        Self {
            cells: Box::new([]),
            number,
            number_float,
        }
    }
}

impl<T: Float, S: Neuron<T>> Bundle<T, S> {
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

impl<'a, T: Float> Bundle<T, OutputCell<'a, T>> {
    pub(super) fn new(number: usize) -> Self {
        Self {
            cells: Box::new([]), // TODO: Нужно ли сделать это внутри?
            ..Self::set_number(number)
        }
    }

    // Помещает целевые данные в сеть.
    pub(super) fn set_targets(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_target(&v));
    }
}

impl<'a, T: Float> Bundle<T, InputCell<'a, T>> {
    pub(super) fn new(number: usize) -> Self {
        Self {
            cells: Box::new([]), // TODO: Нужно ли сделать это внутри?
            ..Self::set_number(number)
        }
    }

    // Помещает входные данные в сеть.
    pub(super) fn set_inputs(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].set_value(&v));
    }
}
