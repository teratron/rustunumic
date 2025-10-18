//! # Arctan Loss
//!
//! Calculates the arctan loss, which is the square of the arctangent of the error.

use super::Float;

/// Calculates the Arctan loss for a single value.
///
/// # Arguments
///
/// * `value` - The error value (e.g., predicted - target).
///
/// # Returns
///
/// The square of the arctangent of the error.
pub fn calculate<T: Float>(value: &T) -> T {
    value.atan().powi(2)
}
