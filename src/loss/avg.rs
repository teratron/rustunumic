//! # Average Loss (Mean Absolute Error)
//!
//! Calculates the mean absolute error between predicted and target values.

use super::Float;

/// Calculates the Average (Mean Absolute Error) loss for a single value.
///
/// # Arguments
///
/// * `value` - The error value (e.g., predicted - target).
///
/// # Returns
///
/// The absolute value of the error.
pub fn calculate<T: Float>(value: &T) -> T {
    value.abs()
}
