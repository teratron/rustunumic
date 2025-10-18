//! # Mean Absolute Percentage Error (MAPE) Loss
//!
//! Calculates the mean absolute percentage error between predicted and target values.
//! NOTE: This loss function requires both predicted and target probabilities.
//! The current library structure for loss calculation only provides the error term
//! (e.g., predicted - target) to the individual loss functions. Implementing MAPE
//! correctly requires access to both the predicted value (y_pred) and the true target value (y_true).
//! This would require a change to the core loss API, for example, changing the `calculate`
//! function signature to accept both values.
//!
//! The formula for MAPE is: `(|y_true - y_pred| / |y_true|) * 100%`
//!
//! This file serves as a placeholder to acknowledge the loss function.
//! A full implementation would require modifying the `get_loss` function in `mod.rs`
//! to pass both predicted and target values down to the individual loss calculators.

use super::Float;

/// Placeholder function that cannot correctly implement MAPE with the current API.
pub fn calculate<T: Float>(_value: &T, _target_placeholder: &T) -> T {
    // This is not a real MAPE calculation.
    // It simply returns zero to satisfy the compiler and indicate the limitation.
    // The `_target_placeholder` is used to match the signature required by `mod.rs`
    // when calling `mape::calculate(value, &T::ONE)`.
    T::ZERO
}
