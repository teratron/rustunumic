use super::cell::NeuronTrait;
use super::FloatTrait;

pub(super) struct Neurons<T, S> {
    // Reference to a slice of output neurons.
    //neurons: Vec<Box<dyn NeuronTrait<T>>>,
    pub(super) neurons: Vec<Box<S>>,

    // Number of output neurons.
    number: usize,
    pub(super) number_float: T,
}

impl<T: FloatTrait, S: NeuronTrait<T>> Neurons<T, S> {
    pub fn new(number: usize) -> Self {
        Self {
            neurons: Vec::new(),
            number,
            number_float: T::from(number as f64), //self.get_number_float(),
        }
    }

    /*pub fn get_number_float(&self) -> T {
        T::from(self.number as f64)
    }*/
}
