//! # Root Mean Squared Error (RMSE) Loss
//!
//! Calculates the root mean squared error between predicted and target values.
//! The square root is applied to the total averaged squared error.

use super::Float;

/// Calculates the component for Root Mean Squared Error (Squared Error).
/// The square root is applied to the total loss after averaging.
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
