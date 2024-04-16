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

pub(crate) struct InputBundle<'a, T> {
    cells: Vec<InputCell<'a, T>>,
    number: usize,
}

impl<'a, T> InputBundle<'a, T> {
    pub(crate) fn new(data: &[T]) -> Self {
        let number = data.len();
        Self {
            cells: Vec::with_capacity(number),
            number,
        }
    }

    pub(crate) fn set_input_data(&mut self, data: &[T]) {
        data.iter()
            .enumerate()
            .for_each(|v| self.cells[v.0].0 = v.1);
    }
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
