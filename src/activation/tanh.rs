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
    value.float_tanh()
}

/// Hyperbolic tangent activation function derivative.
///
/// # Arguments
///
/// * `value` - The last state of the activation function (output of the forward pass).
///
/// # Returns
///
/// The derivative of the last state of the activation function.
pub(super) fn derivative<T: Float>(value: T) -> T {
    // Derivative of tanh(x) is 1 - tanh(x)^2.
    // The function takes the input value, so we need to calculate tanh(value) first.
    T::ONE - activation(value).float_powi(2)
}
