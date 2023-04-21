use enum_iterator::{all, Sequence};

// Activation Mode.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Sequence)]
pub enum Activation {
    // LINEAR - Linear/identity (0).
    LINEAR = 0,

    // RELU - ReLu (rectified linear unit) (1).
    RELU,

    // LEAKY_RELU - Leaky ReLu (leaky rectified linear unit) (2).
    LEAKYRELU,

    // SIGMOID - Logistic, a.k.a. sigmoid or soft step (3).
    SIGMOID,

    // TANH - TanH (hyperbolic tangent) (4).
    TANH,
    //DEFAULT = Activation::SIGMOID,
}

/*impl Activation {
    fn init(&self) {}

    fn check_activation_mode(&self, value: u8) -> u8 {
        value
        // return (
        //     self.DEFAULT_ACTIVATION_MODE
        // if value < self.LINEAR or value > self.TANH
        // else value
        // )
    }

    // def _get_activation(self, value: float) -> float:
    // """Activation function."""
    // return get_activation(value, self._activation_mode)
    //
    // def _get_derivative(self, value: float) -> float:
    // """Derivative activation function."""
    // return get_derivative(value, self._activation_mode)
}

enum FloatEnum {
    F32(f32),
    F64(f64),
}*/

// Activation function.
//pub fn get_activation<T>(value: T, mode: Activation) -> T {
pub fn get_activation(mut value: f32, mode: Activation) -> f32 {
    match mode {
        Activation::LINEAR => value,
        Activation::RELU => {
            if value < 0.0 {
                return 0.0;
            }
            value
        }
        Activation::LEAKYRELU => {
            if value < 0.0 {
                return 0.01 * value;
            }
            value
        }
        Activation::TANH => {
            value = (2.0 * value).exp();
            (value - 1.0) / (value + 1.0)
        }
        Activation::SIGMOID => 1.0 / (1.0 + -value.exp()),
        //_ => get_activation(value, Activation::SIGMOID),
    }
}

// Derivative activation function.
pub fn get_derivative(value: f32, mode: Activation) -> f32 {
    match mode {
        Activation::LINEAR => 1.0,
        Activation::RELU => {
            if value < 0.0 {
                return 0.0;
            }
            1.0
        }
        Activation::LEAKYRELU => {
            if value < 0.0 {
                return 0.01;
            }
            1.0
        }
        Activation::TANH => 1.0 - value.powf(2.0),
        Activation::SIGMOID => value * (1.0 - value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_activation() {
        for mode in all::<Activation>().collect::<Vec<_>>() {
            println!("{:?}", mode as u8);
            let result = get_activation(0.1, mode);
            assert_eq!(result, 0.1, "failed test");
        }
        //let result = get_activation(0.1, Activation::RELU);
        //assert_eq!(result, 0.10, "failed test");
    }
}
