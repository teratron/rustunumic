//! # Linear
//!
//!

use super::Float;

/// Linear/identity activation function.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `slope` - The slope of the line (default 1.0).
/// * `offset` - The offset of the line from the origin (default 0.0).
///
/// # Returns
///
/// The weighted sum of the input data.
pub(super) fn activation<T: Float>(value: T, slope: f64, offset: f64) -> T {
    value * T::from(slope) + T::from(offset)
}

/// Linear/identity activation function derivative.
///
/// # Arguments
///
/// * `slope` - The slope of the line (default 1.0).
///
/// # Returns
///
/// The slope of the line `slope`.
pub(super) fn derivative<T: Float>(slope: f64) -> T {
    T::from(slope)
}
