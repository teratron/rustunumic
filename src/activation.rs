// mod activation;

// Activation Mode.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Activation {
    // LINEAR -- Linear/identity (0).
    LINEAR = 0,

    // RELU -- ReLu (rectified linear unit) (1).
    RELU = 1,

    // LEAKY_RELU -- Leaky ReLu (leaky rectified linear unit) (2).
    LEAKYRELU = 2,

    // SIGMOID -- Logistic, a.k.a. sigmoid or soft step (3).
    SIGMOID = 3,

    // TANH -- TanH (hyperbolic tangent) (4).
    TANH = 4,

    //DEFAULT = Self::SIGMOID
}

impl Activation {
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

// Activation function.
pub fn get_activation(value: f32, mode: Activation) -> f32 {
    match mode {
        Activation::LINEAR => value,
        Activation::RELU => {
            if value < 0.0 {
                return 0.0
            }
            value
        }
        Activation::LEAKYRELU => {
            if value < 0.0 {
                return 0.01 * value
            }
            value
        }
        // case Activation.TANH:
        // value = math.exp(2 * value)
        // value = (value - 1) / (value + 1)
        // if math.isnan(value): # TODO:
        // raise ValueError(f"act {__name__}: loss not-a-number value")
        // if math.isinf(value): # TODO:
        // raise ValueError(f"act {__name__}: loss is infinity")
        // return value
        // # value = math.exp(2 * value)
        // # return (value - 1) / (value + 1)
        // case Activation.SIGMOID | _:
        // value = 1 / (1 + math.exp( - value))
        // if math.isnan(value): # TODO:
        // raise ValueError(f"act {__name__}: loss not-a-number value")
        // if math.isinf(value):  # TODO:
        // raise ValueError(f"act {__name__}: loss is infinity")
        // return value
        // # return 1 / (1 + math.exp( - value))
        _ => println!("Error {mode}"),
    }
}

pub fn get_derivative(value: f32, mode: u8) -> f32 {
    //"""Derivative activation function."""
    match mode:
        case
    Activation.LINEAR:
    return 1
    case
    Activation.RELU:
    return 0
    if value < 0 else 1
    case
    Activation.LEAKY_RELU:
    return 0.01
    if value < 0 else 1
    case
    Activation.TANH:
    return 1 - value * *2
    case
    Activation.SIGMOID | _:
    return value * (1 - value)
}