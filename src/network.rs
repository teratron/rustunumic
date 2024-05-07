//! # Network
//!
//!

#![allow(unused)]

use std::fmt::{Debug, Formatter, Result};

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
        Self {
            cells: Vec::new(),
            input: Bundle::<T, InputCell<T>>::new(),
            output: Bundle::<T, OutputCell<T>>::new(),
            hidden: Bundle::<T, HiddenCell<T>>::new(),
        }
    }
}

impl<'a, T: Float> Default for Network<'a, T> {
    fn default() -> Self {
        Self {
            input: Bundle::<T, InputCell<T>>::default(),
            output: Bundle::<T, OutputCell<T>>::default(),
            hidden: Bundle::<T, HiddenCell<T>>::default(),
            ..Self::new()
        }
    }
}

// Debugging.
impl<T> Debug for Network<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Network")
            .field("cells", &"cells")
            .field("input", &"Bundle<InputCell>")
            .field("output", &"Bundle<OutputCell>")
            .field("hidden", &"Bundle<HiddenCell>")
            .finish()
    }
}
