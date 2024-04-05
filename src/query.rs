//! # Query
//!
//!

use super::{Float, Rustunumic};

impl<T: Float> Rustunumic<T> {
    /// Querying dataset.
    pub fn query(&mut self, input: &[T]) -> Vec<T> {
        let output: Vec<T> = vec![T::ZERO; 10];

        if !self.is_init {
            //raise ValueError(f"{__name__}: not initialized")
        }
        //self.input = input;
        self.calculate_values();
        //self.is_query = true

        output
    }
}
/*
    def query(self, data_input: list[float]) -> list[float]:
        """Querying dataset."""
        if not self._params.is_init:
            raise ValueError(f"{__name__}: not initialized")

        self._data_input = data_input
        self._calculate_neurons()
        # self._params.is_query = True
        self.__is_query = True

        return [n.value for n in self._neurons[self._params.last_ind]]
*/
