// Activation Mode.
#[repr(u8)]
#[derive(Debug)]
pub enum Activation {
    // LINEAR - Linear/identity (0).
    LINEAR = 0,

    // RELU - ReLu (rectified linear unit) (1).
    RELU,

    // LEAKYRELU - Leaky ReLu (leaky rectified linear unit) (2).
    LEAKYRELU,

    // SIGMOID - Logistic, a.k.a. sigmoid or soft step (3).
    SIGMOID,

    // TANH - TanH (hyperbolic tangent) (4).
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

// Activation function.
//pub fn get_activation<T>(value: T, mode: Activation) -> T {
//pub fn activation<'a>(&mut'a value: f32, mode: &Activation) -> &'a f32 {
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

// Derivative activation function.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activation() {
        let data: [(f32, Activation, f32); 7] = [
            (0.1, Activation::LINEAR, 0.1),
            (0.1, Activation::RELU, 0.1),
            (-0.1, Activation::RELU, 0.0),
            (0.1, Activation::LEAKYRELU, 0.1),
            (-0.1, Activation::LEAKYRELU, -0.001),
            (0.1, Activation::SIGMOID, 0.52497918747894),
            (0.1, Activation::TANH, 0.099668),
        ];
        for (value, mode, result) in data {
            assert_eq!(activation(value, &mode), result, "{:?} failed test", mode);
        }
    }

    #[test]
    fn test_derivative() {
        let data: [(f32, Activation, f32); 7] = [
            (0.1, Activation::LINEAR, 1.0),
            (0.1, Activation::RELU, 1.0),
            (-0.1, Activation::RELU, 0.0),
            (0.1, Activation::LEAKYRELU, 1.0),
            (-0.1, Activation::LEAKYRELU, 0.01),
            (0.1, Activation::SIGMOID, 0.089999996),
            (0.1, Activation::TANH, 0.99),
        ];
        for (value, mode, result) in data {
            assert_eq!(derivative(value, &mode), result, "{:?} failed test", mode);
        }
    }
}
