//! # Activation
//!
//!

use crate::float::Float;

/// ## Activation mode
///
/// **List of mode:**
///
/// | Mode      | Description                           |
/// |:----------|:--------------------------------------|
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
    // TODO: ELU, SeLU, SWiSH, ELiSH
}

/// Activation function.
pub(super) fn activation<T: Float>(value: T, mode: &Activation) -> T {
    //let ref val: T = *value;
    match mode {
        Activation::Linear => value,
        Activation::ReLU => {
            if value < T::ZERO {
                T::ZERO
            } else {
                value
            }
        }
        Activation::LeakyReLU => {
            if value < T::ZERO {
                T::from(0.01) * value
            } else {
                value
            }
        }
        Activation::TanH => {
            let v = (T::from(2.) * value).exp();
            (v - T::ONE) / (v + T::ONE)
        }
        Activation::Sigmoid => T::from(1.) / (T::from(1.) + (-value).exp()),
    }
}

/* pub fn get_activation<T: Float>(value: &mut T, mode: &Activation) -> T {
    let v: &mut T = value;
    activation(v, mode);
    v
} */

/// Derivative activation function.
pub(super) fn derivative<T: Float>(value: T, mode: &Activation) -> T {
    match mode {
        Activation::Linear => T::ONE,
        Activation::ReLU => {
            if value < T::ZERO {
                T::ZERO
            } else {
                T::ONE
            }
        }
        Activation::LeakyReLU => {
            if value < T::ZERO {
                T::from(0.01)
            } else {
                T::ONE
            }
        }
        Activation::TanH => T::from(1.) - value.powi(2),
        Activation::Sigmoid => T::from(1.) - value,
    }
}

/* pub fn get_derivative<T: Float>(value: &mut T, mode: &Activation) -> T {
    let v: &mut T = value;
    derivative(v, mode);
    v
} */

/* #[cfg(test)]
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
}*/
