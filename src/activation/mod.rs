//! # Activation Functions
//!
//! This module provides various activation functions and their derivatives
//! commonly used in neural networks.

use super::Float;

mod elish;
mod elu;
mod linear;
mod relu;
mod selu;
mod sigmoid;
mod softmax;
mod swish;
mod tanh;

/// Enum representing different activation functions.
#[repr(u8)]
#[derive(Debug)]
pub enum Activation {
    /// Exponential Linear Unit + Sigmoid.
    ELiSH,

    /// Exponential Linear Unit.
    ELU,

    /// Linear/identity.
    Linear,

    /// Leaky Rectified Linear Unit.
    LeakyReLU,

    /// Rectified Linear Unit.
    ReLU,

    /// Scaled Exponential Linear Unit.
    SeLU,

    /// Logistic, a.k.a. sigmoid or soft step.
    Sigmoid,

    /// SoftMax.
    SoftMax,

    /// SWiSH.
    SWiSH,

    /// Hyperbolic Tangent.
    TanH,
}

impl Default for Activation {
    fn default() -> Self {
        Activation::Linear
    }
}

/// Applies the specified activation function to the input value.
///
/// # Arguments
///
/// * `value` - The input value.
/// * `mode` - The activation function to apply.
///
/// # Returns
///
/// The output of the activation function.
pub fn get_activation<T: Float>(value: T, mode: &Activation) -> T {
    match mode {
        Activation::ELiSH => elish::activation(value),
        Activation::ELU => elu::activation(value, 1.0),
        Activation::Linear => linear::activation(value, 1., 0.),
        Activation::LeakyReLU => relu::activation(value, 0.01),
        Activation::ReLU => relu::activation(value, 0.),
        Activation::SeLU => selu::activation(
            value,
            1.0507009873554804934193349852946,
            1.6732632423543772848170429916717,
        ),
        Activation::Sigmoid => sigmoid::activation(value, 1.),
        Activation::SoftMax => softmax::activation(value),
        Activation::SWiSH => swish::activation(value, 1.0), // Default beta for Swish is 1.0
        Activation::TanH => tanh::activation(value),
    }
}

/// Applies the derivative of the specified activation function to the input value.
///
/// # Arguments
///
/// * `value` - The input value (or output of the forward pass, depending on the function).
/// * `mode` - The activation function whose derivative to apply.
///
/// # Returns
///
/// The derivative of the activation function at the given value.
pub fn get_derivative<T: Float>(value: T, mode: &Activation) -> T {
    match mode {
        Activation::ELiSH => elish::derivative(value),
        Activation::ELU => elu::derivative(value, 1.0),
        Activation::Linear => linear::derivative(1.),
        Activation::LeakyReLU => relu::derivative(value, 0.01),
        Activation::ReLU => relu::derivative(value, 0.),
        Activation::SeLU => selu::derivative(
            value,
            1.050709873554804934193349852946,
            1.6732632423543772848170429916717,
        ),
        Activation::Sigmoid => sigmoid::derivative(value, 1.),
        Activation::SoftMax => softmax::derivative(value),
        Activation::SWiSH => swish::derivative(value, 1.0), // Default beta for Swish is 1.0
        Activation::TanH => tanh::derivative(value),
    }
}
