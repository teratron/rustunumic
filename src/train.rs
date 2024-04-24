//! # Train
//!
//!

use super::{Float, Rustunumic};

/// MaxIteration the maximum number of iterations after which training is forcibly terminated.
const MAX_ITERATION: usize = 1_000_000;

impl<T: Float> Rustunumic<'_, T> {
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
    pub fn train(&mut self, input: &[T], target: &[T]) -> (usize, T) {
        // TODO: Result<(usize, T), Box<dyn Error>>
        if !self.is_init {
            if !self.init(input, target) {
                panic!("not initialized");
            }
        }

        let mut loss: T = T::ZERO;
        let mut min_loss: T = T::ONE;
        let mut count: usize = 1; // TODO: 1 or 0?
        let mut min_count: usize = 0;

        while count <= MAX_ITERATION {
            self.calculate_values();
            loss = self.calculate_loss();

            if loss < min_loss {
                // TODO: every 10 iteration!
                min_loss = loss;
                min_count = count;
                //self.__weights = deepcopy(self.weights)

                if loss < T::LOSS_LIMIT {
                    //self.weights = deepcopy(self.__weights);
                    //return (count, loss);
                    break;
                }
            }
            self.calculate_misses().calculate_weights();
            count += 1;
        }

        if min_count > 0 {
            //self.weights = deepcopy(self.__weights);
        }
        (count, loss)
    }
}
