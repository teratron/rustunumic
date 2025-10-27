//! # Verify
//!

use super::{Float, Rustunumic};
use tracing::{debug, error, info, trace};

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
        debug!("Starting verification with {} input and {} target values", input.len(), target.len());
        if !self.is_init {
            debug!("Network not initialized, initializing now");
            if self.init(input, target) {
                self.is_init = true;
                info!("Network initialized for verification");
            } else {
                error!("Failed to initialize network during verification");
                panic!("not initialized");
            }
        }

        self.network.input.set_inputs(input);
        self.network.output.set_targets(target);
        trace!("Input and target values set, starting value calculation");
        self.calculate_values();
        let loss = self.calculate_loss();
        debug!("Verification completed, loss calculated");
        loss
    }
}
