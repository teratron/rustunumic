//! # Input Cell
//!
//!

use super::NeuronBase;

pub(crate) struct InputCell<'a, T>(&'a T);
//pub(crate) struct InputCell<T>(T);

impl<'a, T> InputCell<'a, T> {
    pub(crate) fn new(value: &T) -> Self {
        InputCell(value)
    }
}

impl<'a, T> NeuronBase<T> for InputCell<'a, T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}

struct InputData<'a, T> {
    neurons: &'a [T],
    cells: Vec<InputCell<T>>,
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = InputCell::<f32>::new(0.1);
        assert_eq!(input.0, 0.1);
    }

    #[test]
    fn test_add() {
        let mut input = InputCell::new(0.2);
        input.add(2.);
        assert_eq!(input.0, 3.);
    }

    #[test]
    #[should_panic]
    fn test_add_panics() {
        let mut input = InputCell::new(0.3);
        input.add(f32::INFINITY);
    }

    #[test]
    fn test_get_value() {
        let input = InputCell::<f64>::new(0.1);
        assert_eq!(input.get_value(), &0.1);
    }
}*/
