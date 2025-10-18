//! # Mean Squared Error (MSE) Loss
//!
//! Calculates the mean squared error between predicted and target values.

use super::Float;

/// Calculates the Mean Squared Error loss for a single value.
///
/// # Arguments
///
/// * `value` - The error value (e.g., predicted - target).
///
/// # Returns
///
/// The square of the error.
pub fn calculate<T: Float>(value: &T) -> T {
    value.powi(2)
}
