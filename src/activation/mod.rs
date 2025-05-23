//! # Activation
//!
//!

use super::Float;

mod linear;
mod relu;
mod sigmoid;
mod softmax;
mod swish;
mod tanh;

/// Activation mode
///
/// **Note:**
///
/// - `Linear` is a linear/identity activation function.
/// - `ReLU` is a rectified linear unit activation function.
/// - `LeakyReLU` is a leaky rectified linear unit activation function.
/// - `Sigmoid` is a logistic, a.k.a. sigmoid or soft step activation function.
/// - `TanH` is a hyperbolic tangent activation function.
/// - `SWiSH` is a SWiSH activation function.
/// - `SoftMax` is a SoftMax activation function.
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

    /// SWiSH.
    SWiSH,

    /// SoftMax.
    SoftMax,
    // TODO: ELU, SeLU, ELiSH
}

impl Default for Activation {
    fn default() -> Self {
        Activation::Linear
    }
}

/// Activation function.
pub fn get_activation<T: Float>(value: T, mode: &Activation) -> T {
    match mode {
        Activation::Linear => linear::activation(value, 1., 0.),
        Activation::ReLU => relu::activation(value, 0.),
        Activation::LeakyReLU => relu::activation(value, 0.01),
        Activation::Sigmoid => sigmoid::activation(value, 1., 0.),
        Activation::TanH => tanh::activation(value),
        Activation::SWiSH => swish::activation(value, 0.),
        Activation::SoftMax => softmax::activation(value),
    }
}

/// Derivative activation function.
pub fn get_derivative<T: Float>(value: T, mode: &Activation) -> T {
    match mode {
        Activation::Linear => linear::derivative(1.),
        Activation::ReLU => relu::derivative(value, 0.),
        Activation::LeakyReLU => relu::activation(value, 0.01),
        Activation::Sigmoid => sigmoid::derivative(value, 1., 0.),
        Activation::TanH => tanh::derivative(value),
        Activation::SWiSH => swish::derivative(value, 0., 0.),
        Activation::SoftMax => softmax::derivative(value),
    }
}

/*trait ActivationTrait<T: Float> {
    fn activation(&self, value: &T);
}

struct Linear;

impl<T: Float> ActivationTrait<T> for Linear {
    fn activation(&self, _value: &T) -> () {
        return;
    }
}*/

/* pub fn get_activation<T: Float>(value: &mut T, mode: &Activation) -> T {
    let v: &mut T = value;
    activation(v, mode);
    v
} */

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
