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
    // TODO: ELU, SELU, SWISH, ELiSH
}

/// Activation function.
pub fn activation(value: &f64, mode: &Activation) -> f64 {
    match mode {
        Activation::Linear => *value,
        Activation::ReLU => {
            if *value < 0. {
                0.
            } else {
                *value
            }
        }
        Activation::LeakyReLU => {
            if *value < 0. {
                0.01 * *value
            } else {
                *value
            }
        }
        Activation::TanH => {
            let val = (2. * *value).exp();
            (val - 1.) / (val + 1.)
        }
        Activation::Sigmoid => 1. / (1. + (-*value).exp()),
    }
}

pub(crate) fn get_activation(value: &mut f64, mode: &Activation) {
    match mode {
        Activation::Linear => return,
        Activation::ReLU => {
            if *value < 0. {
                *value = 0.;
            }
        }
        Activation::LeakyReLU => {
            if *value < 0. {
                *value *= 0.01;
            }
        }
        Activation::TanH => {
            *value = (2. * *value).exp();
            *value = (*value - 1.) / (*value + 1.);
        }
        Activation::Sigmoid => *value = 1. / (1. + (-*value).exp()),
    };
}

/// Derivative activation function.
pub fn derivative(value: &f64, mode: &Activation) -> f64 {
    match mode {
        Activation::Linear => 1.,
        Activation::ReLU => {
            if *value < 0. {
                0.
            } else {
                1.
            }
        }
        Activation::LeakyReLU => {
            if *value < 0. {
                0.01
            } else {
                1.
            }
        }
        Activation::TanH => 1. - value.powf(2.),
        Activation::Sigmoid => *value * (1. - *value),
    }
}

pub(crate) fn get_derivative(value: &mut f64, mode: &Activation) {
    match mode {
        Activation::Linear => *value = 1.,
        Activation::ReLU => {
            if *value < 0. {
                *value = 0.;
            } else {
                *value = 1.;
            }
        }
        Activation::LeakyReLU => {
            if *value < 0. {
                *value = 0.01;
            } else {
                *value = 1.;
            }
        }
        Activation::TanH => *value = 1. - value.powf(2.),
        Activation::Sigmoid => *value *= 1. - *value,
    }
}
