//! # Activation
//!
//!

/// ## Activation mode
///
/// **List of mode:**
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_activation() {
        let mut data: [(f64, Activation, f64); 7] = [
            (0.1, Activation::Linear, 0.1),
            (0.1, Activation::ReLU, 0.1),
            (-0.1, Activation::ReLU, 0.0),
            (0.1, Activation::LeakyReLU, 0.1),
            (-0.1, Activation::LeakyReLU, -0.001),
            (0.1, Activation::Sigmoid, 0.52497918747894),
            (0.1, Activation::TanH, 0.09966799462495583),
        ];
        for (value, mode, result) in data.iter_mut() {
            get_activation(value, mode);
            assert_eq!(*value, *result, "{:?} test", *mode);
        }
    }

    #[test]
    fn test_get_derivative() {
        let mut data: [(f64, Activation, f64); 7] = [
            (0.1, Activation::Linear, 1.0),
            (0.1, Activation::ReLU, 1.0),
            (-0.1, Activation::ReLU, 0.0),
            (0.1, Activation::LeakyReLU, 1.0),
            (-0.1, Activation::LeakyReLU, 0.01),
            (0.1, Activation::Sigmoid, 0.09000000000000001),
            (0.1, Activation::TanH, 0.99),
        ];
        for (value, mode, result) in data.iter_mut() {
            get_derivative(value, mode);
            assert_eq!(*value, *result, "{:?} test", *mode);
        }
    }
}
