//! # Activation
//!
//!

/// ## Activation mode
///
/// ### List of mode:
///
/// | Mode      | Description                           |
/// |-----------|---------------------------------------|
/// | Linear    | Linear/identity                       |
/// | ReLU      | Rectified Linear Unit                 |
/// | LeakyReLU | Leaky Rectified Linear Unit           |
/// | Sigmoid   | Logistic, a.k.a. sigmoid or soft step |
/// | TanH      | Hyperbolic Tangent                    |
#[repr(u8)]
#[derive(Debug)]
pub enum Activation {
    /// Linear/identity.
    Linear,

    /// Rectified Linear Unit.
    ReLU,

    /// Leaky Rectified Linear Unit.
    LeakyReLU,

    /// Logistic, a.k.a. sigmoid or soft step.
    Sigmoid,

    /// Hyperbolic Tangent.
    TanH,
}

/// ## Activation function.
///
/// ### Example:
///
/// ```rust
/// let a = Rustunumic::activation(-0.1, Activation::LeakyReLU);
///
/// assert_eq!(-0.001, a);
/// ```
pub fn activation(value: &f64, mode: &Activation) -> f64 {
    match mode {
        Activation::Linear => *value,
        Activation::ReLU => {
            if *value < 0.0 {
                0.0
            } else {
                *value
            }
        }
        Activation::LeakyReLU => {
            if *value < 0.0 {
                0.01 * value
            } else {
                *value
            }
        }
        Activation::TanH => {
            let val = (2.0 * value).exp();
            (val - 1.0) / (val + 1.0)
        }
        Activation::Sigmoid => 1.0 / (1.0 + (-value).exp()),
    }
}

/// ## Derivative activation function.
///
/// ### Example:
///
/// ```rust
/// let d = Rustunumic::derivative(-0.1, Activation::LeakyReLU);
///
/// assert_eq!(0.01, d);
/// ```
pub fn derivative(value: &f64, mode: &Activation) -> f64 {
    match mode {
        Activation::Linear => 1.0,
        Activation::ReLU => {
            if *value < 0.0 {
                0.0
            } else {
                1.0
            }
        }
        Activation::LeakyReLU => {
            if *value < 0.0 {
                0.01
            } else {
                1.0
            }
        }
        Activation::TanH => 1.0 - value.powf(2.0),
        Activation::Sigmoid => value * (1.0 - value),
    }
}
