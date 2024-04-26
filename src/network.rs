//! # Network
//!
//!

use super::{Bundle, Float, InputCell, Neuron};

#[derive(Debug)]
pub(super) struct Network<'a, T> {
    /// All working neurons.
    pub(super) cells: Vec<Box<dyn Neuron<T>>>, //Vec<&'a dyn Neuron<T>>,

    /// Input neurons.
    pub(super) input: Bundle<T, InputCell<'a, T>>,

    /// Output neurons.
    //pub(super) output: Bundle<T, OutputCell<'a, T>>,
    pub(super) output: Bundle<T, &'a dyn Neuron<T>>,

    /// Hidden neurons.
    //pub(super) hidden: Bundle<T, HiddenCell<T>>,
    pub(super) hidden: Bundle<T, &'a dyn Neuron<T>>,
}

impl<T: Float> Network<'_, T> {
    pub(crate) fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl<'a, T: Float> Default for Network<'a, T> {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            input: Bundle::<T, InputCell<T>>::new(1),
            //output: Bundle::<T, OutputCell<T>>::new(1),
            //hidden: Bundle::<T, HiddenCell<T>>::new(2),
            output: Bundle::<T, &'a dyn Neuron<T>>::new(1),
            hidden: Bundle::<T, &'a dyn Neuron<T>>::new(2),
        }
    }
}
