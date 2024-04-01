//! # Interface
//!
//!

use super::Float;
use super::Rustunumic;

pub trait Interface<T> {
    /// Verify
    fn verify(&mut self, input: &[T], target: &[T]) -> T;

    /// Query
    fn query(&mut self, input: &[T]) -> Vec<T>;

    /// Train
    fn train(&mut self, input: &[T], target: &[T]) -> (usize, T);
}

impl<T: Float> Interface<T> for Rustunumic<T> {
    fn verify(&mut self, input: &[T], target: &[T]) -> T {
        //let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        //let slice = &numbers[1..5];
        //Self::new()
        todo!()
    }

    fn query(&mut self, input: &[T]) -> Vec<T> {
        let output: Vec<T> = vec![T::ZERO; 10];
        output
    }

    fn train(&mut self, input: &[T], target: &[T]) -> (usize, T) {
        let count: usize = 0;
        let loss: T = T::ZERO;

        self.calculate_values();
        let loss = self.calculate_loss();

        if loss < T::LOSS_LIMIT {
            let mut min_loss = loss;
            let mut min_count = count;
            //self.__weights = deepcopy(self._weights)

            if loss < T::LOSS_LIMIT {
                //self.weights = deepcopy(self.__weights);
                return (count, loss);
            }
        }

        (count, loss)
    }
}
