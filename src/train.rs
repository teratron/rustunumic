//! # Train
//!
//!

use super::{Float, Rustunumic};
use tracing::{debug, error, info, trace, warn};

/// MaxIteration the maximum number of iterations after which training is forcibly terminated.
const MAX_ITERATION: usize = 1_000_000;

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Training dataset.
    ///
    /// # Arguments
    ///
    /// * `input` - Input dataset.
    /// * `target` - Target dataset.
    ///
    /// # Returns
    ///
    /// * `usize` - Number of iterations.
    /// * `T` - Total error of the output neurons.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustunumic::Rustunumic;
    /// ```
    pub fn train(&mut self, input: &'a [T], target: &'a [T]) -> (usize, T) {
        debug!("Starting training with {} input and {} target values", input.len(), target.len());
        // TODO: Result<(usize, T), Box<dyn Error>>
        if !self.is_init {
            debug!("Network not initialized, initializing now");
            if !self.init(input, target) {
                error!("Failed to initialize network during training");
                panic!("not initialized");
            }
            info!("Network initialized for training");
        }

        let mut loss: T = T::ZERO;
        let mut min_loss: T = T::ONE;
        let mut count: usize = 1; // TODO: 1 or 0?
        let mut min_count: usize = 0;

        debug!("Starting training loop, max iterations: {}", MAX_ITERATION);
        while count <= MAX_ITERATION {
            self.calculate_values();
            loss = self.calculate_loss();

            if loss < min_loss {
                // TODO: every 10 iteration!
                min_loss = loss;
                min_count = count;
                trace!("New minimum loss achieved at iteration {}", min_count);
                //self.__weights = deepcopy(self.weights)

                if loss < T::LOSS_LIMIT {
                    debug!("Target loss threshold reached at iteration {}, stopping training", count);
                    //self.weights = deepcopy(self.__weights);
                    //return (count, loss);
                    break;
                }
            }
            self.calculate_misses().calculate_weights();
            count += 1;
        }

        if min_count > 0 {
            info!("Training completed after {} iterations, minimum loss was recorded at iteration {}", count, min_count);
            //self.weights = deepcopy(self.__weights);
        } else {
            warn!("Training completed after {} iterations, but no improvement in loss was recorded", count);
        }
        (count, loss)
    }
}
