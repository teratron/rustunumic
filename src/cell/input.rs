//! # Input Cell
//!
//!

#![allow(dead_code)]

//use std::fmt::Debug;

use super::Nucleus;

//#[derive(Debug)]
pub(crate) struct InputCell<'a, T>(&'a T);

impl<'a, T> InputCell<'a, T> {
    pub(crate) fn new(value: &'a T) -> Self {
        InputCell(value)
    }

    pub(crate) fn set_value(&mut self, value: &'a T) {
        self.0 = value;
    }
}

impl<T> Nucleus<T> for InputCell<'_, T>
/*where
T: Debug,*/
{
    fn get_value(&self) -> &T {
        &self.0
    }
}

// Debugging.
/*impl<T> Debug for InputCell<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("InputCell").field(&self.0).finish()
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
