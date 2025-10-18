//! TanH
//!
//!

use super::Float;

/// Hyperbolic tangent activation function.
///
/// # Arguments
///
/// * `value` - The input value.
///
/// # Returns
///
/// The weighted sum of the input data.
pub(super) fn activation<T: Float>(value: T) -> T {
    // let v = (T::from(2.) * value).float_exp();
    // (v - T::ONE) / (v + T::ONE)
    value.tanh()
}

/// Hyperbolic tangent activation function derivative.
///
/// # Arguments
///
/// * `value` - The input value.
///
/// # Returns
///
/// The derivative of the hyperbolic tangent function at the given value.
pub(super) fn derivative<T: Float>(value: T) -> T {
    // Derivative of tanh(x) is 1 - tanh(x)^2.
    // The function takes the input value, so we need to calculate tanh(value) first.
    T::ONE - activation(value).powi(2)
}
