//! # Network
//!
//!

use super::{Bundle, HiddenCell, InputCell, Neuron, OutputCell};

#[derive(Debug)]
pub(super) struct Network<'a, T> {
    /// All working neurons.
    pub(super) cells: Vec<Box<dyn Neuron<T>>>, //Vec<&'a dyn Neuron<T>>,

    /// Input neurons.
    pub(super) input: Bundle<T, InputCell<'a, T>>, //InputBundle<'a, T>,

    /// Output neurons.
    pub(super) output: Bundle<T, OutputCell<'a, T>>,

    /// Hidden neurons.
    pub(super) hidden: Bundle<T, HiddenCell<T>>,
}

impl<'a, T> Network<'a, T> {
    pub(crate) fn new() -> Self {
        Network { ..Self::default() }
    }
}

impl<T> Default for Network<'_, T> {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            input: Bundle::<T, InputCell<T>>::new(1),
            output: Bundle::<T, OutputCell<T>>::new(1),
            hidden: Bundle::<T, HiddenCell<T>>::new(2),
        }
    }
}
