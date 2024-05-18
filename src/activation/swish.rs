//! SWiSH
//!
//!

use super::Float;

/// SoftMax activation function.
pub(super) fn activation<T: Float>(value: T, nonlinear: f64) -> T {
    value / ((-T::from(nonlinear) * value).float_exp() + T::ONE)
}

/// SoftMax activation function derivative.
pub(super) fn derivative<T: Float>(value: T, x: f64, nonlinear: f64) -> T {
    // TODO: check if this is correct
    if T::from(x) == T::ZERO {
        T::from(0.5)
    } else {
        let mut v = T::from(nonlinear) * value;
        v + (value * (T::ONE - v)) / T::from(x)
    }
}
