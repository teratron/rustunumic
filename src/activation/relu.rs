//! # ReLU
//!
//!

use super::Float;

/// Rectified linear unit activation function.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `leak` - The leak coefficient (for passing negative values, default 0.0).
///
/// # Returns
///
/// The weighted sum of the input data.
pub(super) fn activation<T: Float>(value: T, leak: f64) -> T {
    if value < T::ZERO {
        value * T::from(leak)
    } else {
        value
    }
}

/// Rectified linear unit activation function derivative.
///
/// # Arguments
///
/// * `value` - The last state of the activation function (output of the forward pass).
/// * `leak` - The leak coefficient (for passing negative values, default 0.0).
///
/// # Returns
///
/// The `leak` coefficient or `1.0`.
pub(super) fn derivative<T: Float>(value: T, leak: f64) -> T {
    if value < T::ZERO {
        T::from(leak)
    } else {
        T::ONE
    }
}
