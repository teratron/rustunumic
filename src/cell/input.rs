//! # Input Cell
//!
//!

#![allow(dead_code)]

use super::Nucleus;

pub(crate) struct InputCell<'a, T>(&'a T);

impl<T> InputCell<'_, T> {
    pub(crate) fn new(value: &T) -> Self {
        InputCell(value)
    }

    pub(crate) fn set_input(&mut self, input: &T) {
        self.0 = input;
    }
}

impl<T> Nucleus<T> for InputCell<'_, T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}

/*pub(crate) struct InputBundle<'a, T> {
    cells: Vec<InputCell<'a, T>>,
    number: usize,
}

impl<'a, T> InputBundle<'a, T> {
    // Создает объект перечня входных нейронов.
    pub(crate) fn new(data: &'a [T]) -> Self {
        let number = data.len();
        Self {
            cells: Vec::with_capacity(number),
            number,
        }
    }

    // Помещает входные данные в сеть.
    pub(crate) fn set_inputs(&mut self, data: &'a [T]) {
        data.iter()
            .enumerate()
            .for_each(|(i, v)| self.cells[i].0 = v);
    }
}*/

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
