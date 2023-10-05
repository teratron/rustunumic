use rustunumic::activation::*;

#[test]
fn test_activation() {
    let data: [(f64, Activation, f64); 7] = [
        (0.1, Activation::Linear, 0.1),
        (0.1, Activation::ReLU, 0.1),
        (-0.1, Activation::ReLU, 0.0),
        (0.1, Activation::LeakyReLU, 0.1),
        (-0.1, Activation::LeakyReLU, -0.001),
        (0.1, Activation::Sigmoid, 0.52497918747894),
        (0.1, Activation::TanH, 0.09966799462495583),
    ];
    for (value, mode, result) in data {
        assert_eq!(activation(&value, &mode), result, "{:?} failed test", mode);
    }
}

#[test]
fn test_derivative() {
    let data: [(f64, Activation, f64); 7] = [
        (0.1, Activation::Linear, 1.0),
        (0.1, Activation::ReLU, 1.0),
        (-0.1, Activation::ReLU, 0.0),
        (0.1, Activation::LeakyReLU, 1.0),
        (-0.1, Activation::LeakyReLU, 0.01),
        (0.1, Activation::Sigmoid, 0.089999996),
        (0.1, Activation::TanH, 0.99),
    ];
    for (value, mode, result) in data {
        assert_eq!(derivative(&value, &mode), result, "{:?} failed test", mode);
    }
}
