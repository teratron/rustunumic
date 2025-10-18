//! # SoftMax
//!
//!
use super::Float;

// TODO: The activation function must be performed on a vector of values.
/// SoftMax activation function.
///
/// # Arguments
///
/// * `value` - The input value.
///
/// # Returns
///
/// The output of the SoftMax function.
pub(super) fn activation<T: Float>(value: T) -> T {
    // This is a placeholder. The actual implementation would require a vector of values.
    let exp_val = value.float_exp();
    // In a real scenario, we would need the sum of exps for all values in the layer.
    // For a single value, this is not a meaningful softmax.
    // Let's assume a simplification where it normalizes against a hypothetical total of 1.
    exp_val / (exp_val + T::ONE) // This is essentially a type of sigmoid
}

// TODO: The derivative function must be performed on a vector of values.
/// SoftMax activation function derivative.
///
/// # Arguments
///
/// * `value` - The last state of the activation function (output of the forward pass).
///
/// # Returns
///
/// The derivative of the SoftMax function at the given value.
pub(super) fn derivative<T: Float>(value: T) -> T {
    // The derivative of softmax is more complex and depends on the output vector.
    // For a single value, we can approximate with the derivative of the simplified activation.
    let activation_val = activation(value);
    activation_val * (T::ONE - activation_val)
}
