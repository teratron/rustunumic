#![allow(unused)]

use std::time;

use rustunumic::{Activation, Loss, Rustunumic};

struct Perceptron<T> {
    bias: bool,
    rate: T,
    hidden_layers: Vec<usize>,
    activation: Activation,
    loss: Loss,
}

fn main() {
    /*
    bias=True,
    hidden_layers=[5, 3],
    activation_mode=.TANH,
    loss_mode=.MSE,
    loss_limit=1e-6,
    rate=0.3,
    */

    // Returns a new neural network instance.
    let mut rn = Rustunumic::<f32>::new();

    // Dataset.
    let dataset = [
        0.27, -0.31, -0.52, 0.66, 0.81, -0.13, 0.2, 0.49, 0.11, -0.73, 0.28,
    ];
    let len_input = 3; // Number of input data
    let len_output = 2; // Number of output data
    let len_data = dataset.len() - len_output + 1;

    // Start time.
    let now = time::Instant::now();

    // Training.
    for _ in 10_000 {
        for i in len_input..len_data {
            let (num, loss) = rn.train(&dataset[i - len_input..i], &dataset[i..i + len_output]);
        }

        // Verifying.
        let mut sum = 0.;
        let mut num = 0.;
        for i in (len_input..len_data) {
            sum += rn.verify(&dataset[i - len_input..i], &dataset[i..i + len_output]);
            num += 1.;
        }

        // Exiting the cycle of learning epochs, when the minimum error level is reached.
        if sum / num < 1E-10 {
            //rn.loss_limit
            break;
        }
    }

    // Elapsed time.
    println!("Elapsed time: {} seconds.", now.elapsed().as_secs());

    // Check the trained data, the result should be about [-0.13 0.2].
    print!("Check: {:?}", rn.query(&dataset[2..4])); // -0.52, 0.66, 0.81

    //println!("{:#?} {:#?}", num, loss);
    //println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)

    /*let perceptron = Perceptron::<f64> {
        bias: true,
        rate: 0.3,
        hidden_layers: vec![3, 2],
        activation: Activation::TanH,
        loss: Loss::MSE,
    };*/
}
