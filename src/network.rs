//! # Network
//!
//!

#![allow(unused)]

use super::{Bundle, Float, HiddenCell, InputCell, Neuron, OutputCell};

//#[derive(Debug)]
pub(super) struct Network<'a, T> {
    /// All working neurons.
    pub(super) cells: Vec<Box<dyn Neuron<T>>>,

    /// Input neurons.
    pub(super) input: Bundle<T, InputCell<'a, T>>,

    /// Output neurons.
    pub(super) output: Bundle<T, OutputCell<'a, T>>,

    /// Hidden neurons.
    pub(super) hidden: Bundle<T, HiddenCell<T>>,
}

impl<T: Float> Network<'_, T> {
    pub(super) fn new() -> Self {
        Self::default()
    }
}

impl<'a, T: Float> Default for Network<'a, T> {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            input: Bundle::<T, InputCell<T>>::new(),
            output: Bundle::<T, OutputCell<T>>::new(),
            hidden: Bundle::<T, HiddenCell<T>>::new(),
        }
    }
}
