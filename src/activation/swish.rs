//! SWiSH
//!
//!

use super::Float;

/// Swish activation function.
/// ```
/// f(x, &#946;) = x * sigmoid(&#946;x)
/// ```
///
/// # Arguments
///
/// * `value` - The input value.
/// * `beta` - The beta parameter (a.k.a. nonlinear, default 1.0).
///
/// # Returns
///
/// The output of the Swish function.
pub(super) fn activation<T: Float>(value: T, beta: f64) -> T {
    let beta_val = T::from(beta);
    value / ((-beta_val * value).float_exp() + T::ONE)
}

/// Swish activation function derivative.
/// ```
/// f'(x, &#946;) = f(x, &#946;) + sigmoid(&#946;x) * (1 - f(x, &#946;))
/// ```
///
/// # Arguments
///
/// * `value` - The input value.
/// * `beta` - The beta parameter (a.k.a. nonlinear, default 1.0).
///
/// # Returns
///
/// The derivative of the Swish function at the given value.
pub(super) fn derivative<T: Float>(value: T, beta: f64) -> T {
    let beta_val = T::from(beta);
    let sigmoid_beta_x = T::ONE / ((-beta_val * value).float_exp() + T::ONE);
    let swish_val = value * sigmoid_beta_x;
    swish_val + sigmoid_beta_x * (T::ONE - swish_val)
}
