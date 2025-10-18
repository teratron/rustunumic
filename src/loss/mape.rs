//! # Mean Absolute Percentage Error (MAPE) Loss
//!
//! Provides functionality to calculate the Mean Absolute Percentage Error (MAPE)
//! between predicted and target values.
//!
//! The formula for MAPE is: `(|y_true - y_pred| / |y_true|) * 100%`
//!
//! **Important Note on Integration:**
//! The current library structure for loss calculation typically provides only the error term
//! (e.g., `predicted - target`) to individual loss functions via the `calculate` method used by `get_total_loss`.
//! MAPE, however, requires access to both the predicted value (`y_pred`) and the true target value (`y_true`)
//! to be calculated correctly.
//!
//! Therefore, a full integration of this MAPE calculation into the standard `get_total_loss(Loss::MAPE, ...)`
//! workflow would require a modification to the core loss API. This would involve changing how loss functions
//! are called to pass the necessary `predicted` and `target` values, rather than just the pre-calculated error.
//!
//! This module provides a standalone `calculate_mape` function that can be used if you have direct access
//! to the predicted and target values.

use super::Float;

/// Calculates the Mean Absolute Percentage Error (MAPE) for a single prediction-target pair.
///
/// Formula: `(|y_true - y_pred| / |y_true|) * 100%`
///
/// # Arguments
///
/// * `predicted` - The predicted value (y_pred).
/// * `target` - The true target value (y_true).
///
/// # Returns
///
/// The MAPE value for this pair, expressed as a percentage (e.g., 50.0 for 50%).
/// Returns `T::ZERO` if the target is zero to avoid division by zero.
///
/// # Examples
///
/// ```rust
/// use rustunumic::loss::mape;
///
/// let predicted = 110.0_f64;
/// let target = 100.0_f64;
/// let mape_value = mape::calculate(&predicted, &target);
///
/// // MAPE = (|100 - 110| / |100|) * 100 = (10/100)*100 = 10%
/// assert_eq!(mape_value, 10.0);
/// ```
pub fn calculate<T: Float>(predicted: &T, target: &T) -> T {
    let abs_target = target.abs();
    if abs_target == T::ZERO {
        // Avoid division by zero. Depending on context, returning 0.0 or a large value might be appropriate.
        // Here, we return 0.0 as a common choice, though it masks the issue of a zero target.
        return T::ZERO;
    }
    let absolute_error = (*target - *predicted).abs();
    (absolute_error / abs_target) * T::from_f64(100.0)
}
