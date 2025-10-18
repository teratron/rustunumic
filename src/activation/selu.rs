//! # SeLU
//!
//! Scaled Exponential Linear Unit.

use super::Float;

/// Scaled Exponential Linear Unit activation function.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `scale` - The scale parameter (default 1.0507).
/// * `alpha` - The alpha parameter (default 1.6733).
///
/// # Returns
///
/// The output of the SeLU function.
pub(super) fn activation<T: Float>(value: T, scale: f64, alpha: f64) -> T {
    if value < T::ZERO {
        T::from_f64(scale) * (T::from_f64(alpha) * (value.exp() - T::ONE))
    } else {
        T::from_f64(scale) * value
    }
}

/// Scaled Exponential Linear Unit activation function derivative.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `scale` - The scale parameter (default 1.0507).
/// * `alpha` - The alpha parameter (default 1.6733).
///
/// # Returns
///
/// The derivative of the SeLU function at the given value.
pub(super) fn derivative<T: Float>(value: T, scale: f64, alpha: f64) -> T {
    if value < T::ZERO {
        // Derivative for x < 0 is scale * alpha * exp(x)
        T::from_f64(scale) * T::from_f64(alpha) * value.exp()
    } else {
        // Derivative for x >= 0 is scale
        T::from_f64(scale)
    }
}
