#![allow(unused)]

use rustunumic::Rustunumic;

const JSON_STREAM: &'static str = r#"{
    "bias": true,
    "activation_mode": 3,
    "loss_mode": 0,
    "loss_limit": 1e-10,
    "rate": 0.3,
    "weights": [
        [
            [-2.5128086, 2.6974556, 3.034397, -2.4341068],
            [-1.2436904, -1.1729956, 4.4001436, -2.1053333],
            [-1.2884712, 2.5544305, 10.886107, -2.1163273],
            [3.9765725, 3.646633, 4.741202, -3.8852577],
            [8.725591, 3.0480642, 3.0672483, -7.115494]
        ],
        [
            [3.7148979, 2.9444046, -5.72786, 2.2840204, -1.6592604, 0.33781952],
            [1.8408697, 2.070344, -6.0672054, 3.9654624, -2.7668004, 2.3363395],
            [1.8098677, 2.2063692, 0.08325871, -4.959725, 5.3901534, 1.0965135]
        ],
        [
            [2.1007898, 6.552546, -5.262143, -1.1054513],
            [-6.4693666, -4.019415, -3.8858104, 6.2537074]
        ]
    ]
}"#;

fn main() {
    // Returns a new neural network instance.
    let mut rn = Rustunumic::<f32>::new(/*JSON_STREAM*/);

    // Input dataset.
    let data_input = [0.27, 0.31, 0.52];

    // Getting the results of the trained network.
    let data_output = rn.query(&data_input);
    print!("Query: {data_output:?}");
}
