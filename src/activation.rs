pub enum Activation {
    // Activation Mode:
    LINEAR = 0,
    // LINEAR -- Linear/identity (0).
    RELU = 1,
    // RELU -- ReLu (rectified linear unit) (1).
    LEAKYRELU = 2,
    // LEAKY_RELU -- Leaky ReLu (leaky rectified linear unit) (2).
    SIGMOID = 3,
    // SIGMOID -- Logistic, a.k.a. sigmoid or soft step (3).
    TANH = 4, // TANH -- TanH (hyperbolic tangent) (4).
}

impl Activation {
    fn check_activation_mode(self, value: u8) -> u8 {
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
