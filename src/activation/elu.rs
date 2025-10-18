//! # ELU
//!
//! Exponential Linear Unit.

use super::Float;

/// Exponential Linear Unit activation function.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `alpha` - The alpha parameter (default 1.0).
///
/// # Returns
///
/// The output of the ELU function.
pub(super) fn activation<T: Float>(value: T, alpha: f64) -> T {
    if value < T::ZERO {
        T::from_f64(alpha) * ((value.exp()) - T::ONE)
    } else {
        value
    }
}

/// Exponential Linear Unit activation function derivative.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `alpha` - The alpha parameter (default 1.0).
///
/// # Returns
///
/// The derivative of the ELU function at the given value.
pub(super) fn derivative<T: Float>(value: T, alpha: f64) -> T {
    if value < T::ZERO {
        // Derivative for x < 0 is alpha * exp(x)
        T::from_f64(alpha) * value.exp()
    } else {
        // Derivative for x >= 0 is 1
        T::ONE
    }
}
