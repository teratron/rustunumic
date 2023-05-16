use rustunumic::activation::*;

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
        assert_eq!(activation(&value, &mode), result, "{:?} failed test", mode);
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
        assert_eq!(derivative(&value, &mode), result, "{:?} failed test", mode);
    }
}
