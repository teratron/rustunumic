use super::{Float, Neuron};

pub(super) struct Network<T, S> {
    /// Reference to a slice of output or hidden neurons.
    pub(super) neurons: Box<[S]>,

    /// Number of output neurons.
    pub(super) number: usize,
    pub(super) number_float: T,
}

impl<T: Float, S: Neuron<T>> Network<T, S> {
    pub fn new(number: usize) -> Self {
        Self {
            neurons: Box::new([]),
            number,
            number_float: T::from(number as f64), //self.get_number_float(),
        }
    }

    /*pub fn get_number_float(&self) -> T {
        T::from(self.number as f64)
    }*/
}
