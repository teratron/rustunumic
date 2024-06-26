//! # Verify
//!

use super::{Float, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Verifying dataset.
    ///
    /// # Arguments
    ///
    /// * `input` - Input dataset.
    /// * `target` - Target dataset.
    ///
    /// # Returns
    ///
    /// * `T` - Total error of the output neurons.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustunumic::Rustunumic;
    /// ```
    pub fn verify(&mut self, input: &'a [T], target: &'a [T]) -> T {
        if !self.is_init {
            if self.init(input, target) {
                self.is_init = true;
            } else {
                panic!("not initialized");
            }
        }

        self.network.input.set_inputs(input);
        self.network.output.set_targets(target);
        self.calculate_values();
        self.calculate_loss()
    }
}
