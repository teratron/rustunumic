//! # Sigmoid
//!
//!

use super::Float;

/// Logistic, a.k.a. sigmoid or soft step activation function.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `slope` - The slope of the line in the range from 0.0 to 1.0 (default 1.0).
/// * `offset` - The offset of the line from the origin (default 0.0).
///
/// # Returns
///
/// The weighted sum of the input data.
pub(super) fn activation<T: Float>(value: T, slope: f64) -> T {
    T::from_f64(slope) / ((-value).exp() + T::ONE)
}

/// Logistic, a.k.a. sigmoid or soft step activation function derivative.
///
/// # Arguments
///
/// * `value` - The last state of the activation function (output of the forward pass).
/// * `slope` - The slope of the line in the range from 0.0 to 1.0 (default 1.0).
///
/// # Returns
///
/// The derivative of the last state of the activation function.
pub(super) fn derivative<T: Float>(value: T, slope: f64) -> T {
    let sigmoid_val = activation(value, slope);
    sigmoid_val * (T::ONE - sigmoid_val / T::from_f64(slope))
}
