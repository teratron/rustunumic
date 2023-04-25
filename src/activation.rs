//! # Activation
//!
//!

/// Activation mode.
///
/// # List of mode:
///
/// * LINEAR - Linear/identity (0);
/// * RELU - ReLu (rectified linear unit) (1);
/// * LEAKYRELU - Leaky ReLu (leaky rectified linear unit) (2);
/// * SIGMOID - Logistic, a.k.a. sigmoid or soft step (3);
/// * TANH - TanH (hyperbolic tangent) (4).
#[repr(u8)]
#[derive(Debug)]
pub enum Activation {
    /// LINEAR - Linear/identity (0).
    LINEAR = 0,

    /// RELU - ReLu (rectified linear unit) (1).
    RELU,

    /// LEAKYRELU - Leaky ReLu (leaky rectified linear unit) (2).
    LEAKYRELU,

    /// SIGMOID - Logistic, a.k.a. sigmoid or soft step (3).
    SIGMOID,

    /// TANH - TanH (hyperbolic tangent) (4).
    TANH,
}

/*enum FloatEnum {
    F32(f32),
    F64(f64),
}*/

/*impl U8 for Activation {}

struct U8<T>
where
    T: u8;

impl U8<u8> {
    fn to_activation(&self) -> Activation {
        Activation::U8(&self)
    }
}*/

/*impl Activation {
    // fn activation_to(value: u8) -> Activation {
    //     Activation::U8(value)
    // }
    // fn to_u8(self) -> u8 {
    //     self as u8
    // }
    // fn init(&self) {}
    //
    // fn check(self, value: Activation) -> u8 {
    //     if value < Self::LINEAR || value > self::TANH {
    //         self::DEFAULT
    //     }
    //     value
    // }

    pub fn get_activation(self, value: f32) -> f32 {
        return get_activation(value, self._activation_mode);
    }

    pub fn get_derivative(self, value: f32) -> f32 {
        return get_derivative(value, self._activation_mode);
    }
}*/

//pub fn get_activation<T>(value: T, mode: Activation) -> T {
//pub fn activation<'a>(&mut'a value: f32, mode: &Activation) -> &'a f32 {
/// Activation function.
///
/// # Examples
///
/// ```rust
/// let activation = Rustunumic::activation(-0.1, Activation::LEAKYRELU);
///
/// assert_eq!(-0.001, activation);
/// ```
pub fn activation(mut value: f32, mode: &Activation) -> f32 {
    match mode {
        Activation::LINEAR => value,
        Activation::RELU => {
            if value < 0.0 {
                0.0
            } else {
                value
            }
        }
        Activation::LEAKYRELU => {
            if value < 0.0 {
                0.01 * value
            } else {
                value
            }
        }
        Activation::TANH => {
            value = (2.0 * value).exp();
            (value - 1.0) / (value + 1.0)
        }
        Activation::SIGMOID => 1.0 / (1.0 + (-value).exp()),
    }
}

/// Derivative activation function.
pub fn derivative(value: f32, mode: &Activation) -> f32 {
    match mode {
        Activation::LINEAR => 1.0,
        Activation::RELU => {
            if value < 0.0 {
                0.0
            } else {
                1.0
            }
        }
        Activation::LEAKYRELU => {
            if value < 0.0 {
                0.01
            } else {
                1.0
            }
        }
        Activation::TANH => 1.0 - value.powf(2.0),
        Activation::SIGMOID => value * (1.0 - value),
    }
}
