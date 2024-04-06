//! # Initialization
//!
//!

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Initialization neural network.
    pub(super) fn init(&mut self, input: &[T], target: &[T]) -> bool {
        self.is_init = true;
        self.is_init
    }
}
