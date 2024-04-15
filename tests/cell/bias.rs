use approx::assert_relative_eq;
use float_eq::assert_float_eq;

use crate::cell::BiasCell;

#[test]
fn test_new() {
    let bias = BiasCell::<f32>::new();
    assert_float_eq!(bias.0, 1.);
}

#[test]
fn test_add() {
    let mut bias = BiasCell::new();
    bias.add(2.);
    assert_relative_eq!(bias.0, 3.);
}

#[test]
#[should_panic]
fn test_add_panics() {
    let mut bias = BiasCell::new();
    bias.add(f32::INFINITY);
}
