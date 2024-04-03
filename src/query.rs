use super::{FloatTrait, Rustunumic};

impl<T: FloatTrait> Rustunumic<T> {
    pub fn query(&mut self, input: &[T]) -> Vec<T> {
        let output: Vec<T> = vec![T::ZERO; 10];
        output
    }
}

/*

*/
