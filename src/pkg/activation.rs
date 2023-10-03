//! # Activation
//!
//!

/// ## Activation mode
///
/// ### List of mode:
///
/// | Code | Activation | Description                              |
/// |------|------------|------------------------------------------|
/// | 0    | Linear     | Linear/identity                          |
/// | 1    | RELU       | ReLu (rectified linear unit)             |
/// | 2    | LeakyRELU  | Leaky ReLu (leaky rectified linear unit) |
/// | 3    | Sigmoid    | Logistic, a.k.a. sigmoid or soft step    |
/// | 4    | TANH       | TanH (hyperbolic tangent)                |
#[repr(u8)]
#[derive(Debug)]
pub enum Activation {
    /// LINEAR - Linear/identity.
    Linear,

    /// RELU - ReLu (rectified linear unit).
    RELU,

    /// LEAKY_RELU - Leaky ReLu (leaky rectified linear unit).
    LeakyRELU,

    /// SIGMOID - Logistic, a.k.a. sigmoid or soft step.
    Sigmoid,

    /// TANH - TanH (hyperbolic tangent).
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
pub fn activation(value: &f32, mode: &Activation) -> f32 {
    match mode {
        Activation::LINEAR => *value,
        Activation::RELU => {
            if *value < 0.0 {
                0.0
            } else {
                *value
            }
        }
        Activation::LEAKY_RELU => {
            if *value < 0.0 {
                0.01 * value
            } else {
                *value
            }
        }
        Activation::TANH => {
            let val = (2.0 * value).exp();
            (val - 1.0) / (val + 1.0)
        }
        Activation::SIGMOID => 1.0 / (1.0 + (-value).exp()),
    }
}

/*mod foo {
trait MyTrait {}
impl MyTrait for f32 {}
impl MyTrait for f64 {}

pub fn f<T: MyTrait>(x: T) {}
}*/

/*pub fn main() {
    foo::f(3f32);
    foo::f(3f64);
    foo::f(3u64);  // doesn't work as u64 doesn't implement MyTrait
}*/

/*mod internal {
    pub trait Real {}
}
use internal::Real;

pub struct Quat<T>([T; 4]) where T: Real;

impl Real for f32 {}
impl Real for f64 {}

#[test]
fn test_thing() {}

union FloatUnion {
    float32: f32,
    float64: f64,
}*/

trait Real {
    type T;

    fn to_real(self) -> Self::T;
    fn to_primitive(self: Self) -> Self::T;
}

/*impl<T> Real for T {
    fn to_real(self) -> T {
        self
    }
}*/

impl Real for f32 {
    type T = f32;

    fn to_real(self) -> Self::T {
        self
    }
    fn to_primitive(self: Self::T) -> f32 {
        self
    }
}

impl Real for f64 {
    type T = f64;

    fn to_real(self) -> Self::T {
        self
    }
    fn to_primitive(self: Self::T) -> f64 {
        self
    }
}

/// Derivative activation function.
pub fn derivative(value: &f32, mode: &Activation) -> f32 {
    //pub fn derivative<T: Real<T>>(value: T, mode: &Activation) -> T {
    //pub fn derivative<T>(value: &T, mode: &Activation) -> T {
    //let val = value.to_primitive();
    match mode {
        Activation::LINEAR => 1.0, //.to_real(),
        Activation::RELU => {
            if *value < 0.0 {
                0.0 //.to_real()
            } else {
                1.0 //.to_real()
            }
        }
        Activation::LEAKY_RELU => {
            if *value < 0.0 {
                0.01
            } else {
                1.0
            }
        }
        Activation::TANH => 1.0 - value.powf(2.0),
        Activation::SIGMOID => value * (1.0 - value),
    }
}