//! # Sigmoid
//!
//!

use super::Float;

/// Logistic, a.k.a. sigmoid or soft step activation function.
pub(super) fn activation<T: Float>(value: T, slope: f64, offset: f64) -> T {
    //T::from(1.) / ((-value).float_exp() + T::ONE)
    T::from(slope) / ((-value).float_exp() + T::ONE - T::from(offset))
}

/// Logistic, a.k.a. sigmoid or soft step activation function derivative.
pub(super) fn derivative<T: Float>(value: T, slope: f64, offset: f64) -> T {
    //T::from(1.) - value
    let mut v = (value + T::from(offset)).float_min(slope).float_max(0.);
    v * (T::ONE - v / T::from(slope))
}
