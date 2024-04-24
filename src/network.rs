//! # Network
//!
//!

#![allow(dead_code)]

use super::Bundle;
use super::Neuron;
use super::{HiddenCell, InputCell, OutputCell};

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
        todo!()
        /*Network {
            cells: Vec::new(),
            input: (),
            output: Bundle::new(5),
            hidden: Bundle {},
        }*/
    }
}
