use rustunumic::activation::{Activation, get_activation, get_derivative};

#[test]
fn test_activation() {
    let data: [(f64, Activation, f64); 14] = [
        (0.1, Activation::Linear, 0.1),
        (0.1, Activation::ReLU, 0.1),
        (-0.1, Activation::ReLU, 0.0),
        (0.1, Activation::LeakyReLU, 0.1),
        (-0.1, Activation::LeakyReLU, -0.001),
        (0.1, Activation::Sigmoid, 0.52497918747894),
        (0.1, Activation::TanH, 0.09966799462495582), // Corrected precision
        (0.1, Activation::ELiSH, 0.052497918747894),
        (-0.1, Activation::ELU, -0.09516258196404048),
        (0.1, Activation::ELU, 0.1),
        (0.1, Activation::SeLU, 0.10507009873554805),
        (-0.1, Activation::SeLU, -0.16730527262431402),
        (0.1, Activation::SoftMax, 0.52497918747894),
        (0.1, Activation::SWiSH, 0.052497918747894),
    ];
    for (value, mode, result) in data {
        assert_eq!(
            get_activation(value, &mode),
            result,
            "{:?} failed test",
            mode
        );
    }
}

#[test]
fn test_derivative() {
    let data: [(f64, Activation, f64); 14] = [
        (0.1, Activation::Linear, 1.0),
        (0.1, Activation::ReLU, 1.0),
        (-0.1, Activation::ReLU, 0.0),
        (0.1, Activation::LeakyReLU, 1.0),
        (-0.1, Activation::LeakyReLU, 0.01),
        (0.1, Activation::Sigmoid, 0.24937604019289197), // Corrected value
        (0.1, Activation::TanH, 0.9900662908474398),     // Corrected value
        (0.1, Activation::ELiSH, 0.5499167914982293),
        (-0.1, Activation::ELU, 0.9048374180359595),
        (0.1, Activation::ELU, 1.0),
        (0.1, Activation::SeLU, 1.0507098735548048),
        (-0.1, Activation::SeLU, 1.5908075222060172),
        (0.1, Activation::SoftMax, 0.24937604019289197),
        (0.1, Activation::SWiSH, 0.5499167914982293),
    ];
    for (value, mode, result) in data {
        assert_eq!(
            get_derivative(value, &mode),
            result,
            "{:?} failed test",
            mode
        );
    }
}
