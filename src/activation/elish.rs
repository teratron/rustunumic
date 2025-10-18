//! # ELiSH
//!
//! Exponential Linear Unit + Sigmoid.
//!
//! ```
//! f(x) = x >= 0 ? x * sigmoid(x) : (exp(x) - 1) * sigmoid(x)
//! ```

use super::Float;

/// Exponential Linear Unit + Sigmoid activation function.
///
/// # Arguments
///
/// * `value` - The input value.
///
/// # Returns
///
/// The output of the ELiSH function.
pub(super) fn activation<T: Float>(value: T) -> T {
    let sigmoid_val = T::ONE / (T::ONE + (-value).exp());
    if value < T::ZERO {
        (value.exp() - T::ONE) * sigmoid_val
    } else {
        value * sigmoid_val
    }
}

/// Exponential Linear Unit + Sigmoid activation function derivative.
///
/// # Arguments
///
/// * `value` - The input value (not the output of forward pass).
///
/// # Returns
///
/// The derivative of the ELiSH function at the given value.
pub(super) fn derivative<T: Float>(value: T) -> T {
    let sigmoid_val = T::ONE / (T::ONE + (-value).exp());
    let sigmoid_deriv = sigmoid_val * (T::ONE - sigmoid_val);
    if value < T::ZERO {
        let exp_val = value.exp();
        // d/dx (e^x - 1) * sigmoid(x) = e^x * sigmoid(x) + (e^x - 1) * sigmoid'(x)
        exp_val * sigmoid_val + (exp_val - T::ONE) * sigmoid_deriv
    } else {
        // d/dx x * sigmoid(x) = sigmoid(x) + x * sigmoid'(x)
        sigmoid_val + value * sigmoid_deriv
    }
}
