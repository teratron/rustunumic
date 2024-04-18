//! # Verify
//!

use super::{Float, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Verifying dataset.
    pub fn verify(&mut self, input: &'a [T], target: &'a [T]) -> T {
        if !self.is_init {
            if self.init(input, target) {
                self.is_init = true;
            } else {
                panic!("not initialized");
            }
        }

        self.input_cells.set_input_data(input);
        self.output_cells.set_target_data(target);
        T::ZERO
    }
}

/*
    def verify(self, data_input: list[float], data_target: list[float]) -> float:
        """Verifying dataset."""
        if not self._params.is_init:
            if self.__init(len(data_input), len(data_target)):
                self._params.is_init = True

        self._data_input = data_input
        self._data_target = data_target
        self._calculate_neurons()

        # noinspection PyArgumentList
        return self._calculate_loss()
*/
